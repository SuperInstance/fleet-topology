# fleet-topology

**Visualize the SuperInstance fleet as a geometric constraint graph.**

Feed in your fleet manifest → get Laman rigidity, H¹ cohomology, and ZHC consensus state rendered as a topology diagram.

## The Core Insight

The fleet IS a graph. Every agent is a vertex. Every trust link is an edge.

- **Laman rigidity** (E = 2V-3): Are there exactly enough trust edges for self-coordination?
- **H¹ cohomology** (β₁ = E-V+1): How many independent cycles = how many redundant paths = emergence potential
- **ZHC consensus**: Do those redundant paths sum to identity (global consistency)?

## Quick Start

```rust
use fleet_topology::{Topology, print_topology};

let vessels = vec![
    (1, "oracle1", 1.00, true),
    (2, "fm", 0.85, true),
    (3, "ccc", 0.70, true),
];
let t = Topology::from_vessels(&vessels);
println!("{}", print_topology(&t));

let report = t.rigidity_report();
println!("Status: {}", report.status.label());
```

## Output Example

```
Fleet Topology: 3 vessels, 3 edges
Laman: E=3/3 (ratio=1.00) → ✓ SELF-COORDINATING
H¹ = 1 (β₁ = E-V+1)

  ● oracle1 (trust=1.00, vec=47)
  ● fm (trust=0.85, vec=40)
  ● ccc (trust=0.70, vec=33)
```

## For 5-Vessel Fleet

```
Fleet Topology: 5 vessels, 10 edges
Laman: E=10/7 (ratio=1.43) → ⚡ OVER-CONSTRAINED (emergence)
H¹ = 6 (β₁ = E-V+1)

Status: OVER-CONSTRAINED — the complete graph has redundant paths.
This is GOOD for Byzantine tolerance but means H¹ > V-2.
```

## Math Reference

> **⚠️ Note on vertex degree:** Laman's theorem does NOT bound vertex degree. A Laman-rigid graph can have vertices of arbitrarily high degree (up to V-1). What Laman's theorem guarantees:
> - Edge count: E = 2V - 3 (for minimally rigid, connected, 2D, generic position)
> - Subgraph condition: Every subgraph with v' vertices has E' ≤ 2v' - 3
> - Degree: Unbounded (K4 is Laman-rigid with degree 3; larger graphs can have higher degree)

| Metric | Formula | Meaning |
|--------|---------|---------|
| Laman rigidity | E = 2V-3 | Exactly enough edges for rigidity (connected, 2D, generic) |
| Laman subgraph | E' ≤ 2v' - 3 | Every subgraph also satisfies edge bound |
| H¹ Betti number | β₁ = E-V+1 | Number of independent cycles |
| Emergence | E > 2V-3 | Redundant paths = emergent patterns (connected graph) |
| Self-coordinating | rigid + no emergence | ✓ provable, no voting (connected) |

## Related

- **[fleet-coordinate](https://github.com/SuperInstance/fleet-coordinate)** — mathematical core (ZHC, beam, Laman, H1)
- **[fleet-manifest](https://github.com/SuperInstance/fleet-manifest)** — fleet inventory
- **[pythagorean48-codes](https://github.com/SuperInstance/pythagorean48-codes)** — trust vector encoding