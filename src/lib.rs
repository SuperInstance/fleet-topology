//! fleet-topology — Visualize fleet as a geometric constraint graph
//!
//! Key insight: The fleet IS a graph. Each agent is a vertex. Each trust link
//! is an edge. Laman rigidity (e = 2V-3) tells you when the graph is
//! provably self-coordinating. H1 cohomology (β₁ = e-v+c) tells you how
//! many redundant paths exist (emergence). ZHC consensus tells you when
//! those redundant paths sum to identity (global consistency).
//!
//! This crate provides the visual/analyzer layer on top of fleet-coordinate.

use serde::{Deserialize, Serialize};

/// A node in the fleet graph (one agent)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub name: String,
    pub trust: f64,
    pub trust_vector_idx: u8, // Pythagorean48 index
    pub status: NodeStatus,
    pub x: f64, // Layout position (computed)
    pub y: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Unknown,
}

/// An edge in the fleet graph (one trust link)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub from: u64,
    pub to: u64,
    pub weight: f64,
    pub direction_idx: u8, // Pythagorean48 vector index
}

/// The full fleet topology
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topology {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Topology {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn v(&self) -> usize {
        self.nodes.len()
    }
    pub fn e(&self) -> usize {
        self.edges.len()
    }

    /// Compute Laman rigidity check
    /// Laman-rigid: e = 2V - 3 (within 5% tolerance)
    pub fn laman_rigid(&self) -> (bool, f64) {
        let v = self.v();
        let e = self.e();
        if v < 2 {
            return (false, 0.0);
        }
        let expected = 2 * v - 3; // v≥2  ⇒  expected ≥ 1
        let ratio = e as f64 / expected as f64;
        ((ratio - 1.0).abs() < 0.05, ratio)
    }

    /// H1 cohomology: Betti number β₁ = e - v + c
    pub fn h1_cohomology(&self) -> usize {
        let v = self.v();
        if v == 0 {
            return 0; // the empty graph has zero components, zero cycles
        }
        let e = self.e();
        let c = 1; // assume 1 connected component
        e.saturating_sub(v).saturating_add(c)
    }

    /// Emergence: e > 2V - 3 (strictly over-rigid = emergent patterns)
    /// e = 2V-3 is exactly rigid (Laman) - not over, not under
    pub fn has_emergence(&self) -> bool {
        let v = self.v();
        let e = self.e();
        if v < 3 {
            return false;
        } // Small graphs can't be over-constrained
        let threshold = 2 * v - 3;
        e > threshold
    }

    /// Self-coordinating: rigid + H1 tells you how many redundant paths
    pub fn rigidity_report(&self) -> RigidityReport {
        let (is_rigid, ratio) = self.laman_rigid();
        let h1 = self.h1_cohomology();
        let v = self.v();
        let e = self.e();
        let threshold = 2 * v.saturating_sub(3);
        let emergence = self.has_emergence();

        let status = if v < 2 {
            RigidityStatus::Unknown
        } else if is_rigid && !emergence {
            RigidityStatus::SelfCoordinating
        } else if (is_rigid && emergence) || (!is_rigid && e > threshold) {
            RigidityStatus::OverConstrained
        } else if !is_rigid && e < threshold {
            RigidityStatus::UnderConstrained
        } else {
            RigidityStatus::Unknown
        };

        RigidityReport {
            is_rigid,
            v,
            e,
            laman_expected: threshold,
            ratio,
            h1_cohomology: h1,
            has_emergence: emergence,
            status,
        }
    }

    /// Build from fleet manifest
    pub fn from_vessels(vessels: &[(u64, String, f64, bool)]) -> Self {
        let mut t = Self::new();
        for (id, name, trust, online) in vessels {
            let status = if *online {
                NodeStatus::Online
            } else {
                NodeStatus::Offline
            };
            let trust_vector_idx = ((trust * 47.0) as u8).min(47);
            t.nodes.push(Node {
                id: *id,
                name: name.clone(),
                trust: *trust,
                trust_vector_idx,
                status,
                x: (*id as f64 * 1.3).cos() * 100.0,
                y: (*id as f64 * 1.3).sin() * 100.0,
            });
        }
        // Complete graph assumption for now
        let n = t.nodes.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let w = t.nodes[i].trust.min(t.nodes[j].trust);
                let dir = ((w * 47.0) as u8).min(47);
                t.edges.push(Edge {
                    from: t.nodes[i].id,
                    to: t.nodes[j].id,
                    weight: w,
                    direction_idx: dir,
                });
            }
        }
        t
    }
}

impl Default for Topology {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RigidityReport {
    pub is_rigid: bool,
    pub v: usize,
    pub e: usize,
    pub laman_expected: usize,
    pub ratio: f64,
    pub h1_cohomology: usize,
    pub has_emergence: bool,
    pub status: RigidityStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RigidityStatus {
    SelfCoordinating, // Laman-rigid, no overconstraint
    OverConstrained,  // Laman-rigid but e >> 2V-3 (emergent patterns)
    UnderConstrained, // e < 2V-3 (not enough trust edges)
    Unknown,
}

impl RigidityStatus {
    pub fn label(&self) -> &str {
        match self {
            RigidityStatus::SelfCoordinating => "✓ SELF-COORDINATING",
            RigidityStatus::OverConstrained => "⚡ OVER-CONSTRAINED (emergence)",
            RigidityStatus::UnderConstrained => "✗ UNDER-CONSTRAINED",
            RigidityStatus::Unknown => "? UNKNOWN",
        }
    }
}

/// Output the topology as a text diagram
pub fn print_topology(t: &Topology) -> String {
    let report = t.rigidity_report();
    let mut out = format!(
        "Fleet Topology: {} vessels, {} edges\n\
         Laman: e={}/{} (ratio={:.2}) → {}\n\
         H¹ = {} (β₁ = e-v+1)\n\
         \n",
        t.v(),
        t.e(),
        t.e(),
        report.laman_expected,
        report.ratio,
        report.status.label(),
        report.h1_cohomology,
    );
    for node in &t.nodes {
        let status_icon = match node.status {
            NodeStatus::Online => "●",
            NodeStatus::Offline => "○",
            NodeStatus::Unknown => "?",
        };
        out.push_str(&format!(
            "  {} {} (trust={:.2}, vec={:2})\n",
            status_icon, node.name, node.trust, node.trust_vector_idx
        ));
    }
    out.push_str("Edges:\n");
    for edge in &t.edges {
        out.push_str(&format!(
            "  {} → {} (weight={:.2}, vec={:2})\n",
            edge.from, edge.to, edge.weight, edge.direction_idx
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_topology() {
        let t = Topology::new();
        let report = t.rigidity_report();
        assert_eq!(report.v, 0);
        assert_eq!(report.e, 0);
        assert!(!report.is_rigid);
        assert_eq!(report.h1_cohomology, 0);
        assert!(!report.has_emergence);
        assert_eq!(report.status, RigidityStatus::Unknown);
    }

    #[test]
    fn test_two_nodes_one_edge() {
        let vessels = vec![
            (1, "a".to_string(), 0.8, true),
            (2, "b".to_string(), 0.8, true),
        ];
        let t = Topology::from_vessels(&vessels);
        let report = t.rigidity_report();
        assert!(report.is_rigid);
        assert!(!report.has_emergence);
        assert_eq!(report.status, RigidityStatus::SelfCoordinating);
    }

    #[test]
    fn test_triangle_rigid() {
        // 3 vessels, triangle: v=3, e=3, expected=3, ratio=1.0 → rigid
        let vessels = vec![
            (1, "a".to_string(), 0.8, true),
            (2, "b".to_string(), 0.8, true),
            (3, "c".to_string(), 0.8, true),
        ];
        let t = Topology::from_vessels(&vessels);
        let report = t.rigidity_report();
        assert!(report.is_rigid);
        assert!(!report.has_emergence);
        assert_eq!(report.status, RigidityStatus::SelfCoordinating);
    }

    #[test]
    fn test_five_vessel_fleet() {
        // 5 vessels, complete graph: v=5, e=10, expected=7, ratio=1.43
        let vessels = vec![
            (1, "oracle1".to_string(), 1.00, true),
            (2, "fm".to_string(), 0.85, true),
            (3, "jc1".to_string(), 0.75, false),
            (4, "ccc".to_string(), 0.70, true),
            (5, "probe".to_string(), 0.50, true),
        ];
        let t = Topology::from_vessels(&vessels);
        assert_eq!(t.v(), 5);
        assert_eq!(t.e(), 10);

        let report = t.rigidity_report();
        assert!(!report.is_rigid); // not within 5% of 2V-3
        assert!(report.has_emergence); // e > 2V-3
        assert_eq!(report.h1_cohomology, 6); // e-v+1 = 10-5+1 = 6
        assert_eq!(report.status, RigidityStatus::OverConstrained);
    }

    #[test]
    fn test_print_topology() {
        let vessels = vec![
            (1, "oracle1".to_string(), 1.00, true),
            (2, "fm".to_string(), 0.85, true),
            (3, "ccc".to_string(), 0.70, true),
        ];
        let t = Topology::from_vessels(&vessels);
        let diagram = print_topology(&t);
        assert!(diagram.contains("SELF-COORDINATING") || diagram.contains("OVER-CONSTRAINED"));
    }
}
