# Fleet Topology


## Meta

**Domain:** agent-coordination
**Depends on:** fleet-coordinate
**Depended by:** fleet-homology, fleet-constraint
**Implements:** network-topology, agent-routing, connectivity-graph
**Related:** fleet-coordinate, fleet-manifest, beacon-protocol


**How the fleet is connected. Agent graph, routing topology, and neighbor discovery.**

The fleet is not a flat network. Agents have roles, trust relationships, and physical constraints — some are on the same machine, some communicate over the network, some are on microcontrollers that wake once a minute to relay data.

`fleet-topology` models this: who can talk to whom, what routes exist between any two agents, and which paths are trusted.

---

## What It Provides

**Agent connectivity** — the directed graph of who can reach whom. Edges have weights: latency, bandwidth, trust score.

**Routing topology** — given a destination and a message priority, find the best path. For trusted fleet messages, prefer direct edges. For broadcast messages, use spanning tree.

**Neighbor discovery** — agents discover their neighbors through [beacon-protocol](https://github.com/SuperInstance/beacon-protocol) heartbeats and register their topology in a shared graph. The fleet graph is always up to date within one beacon interval.

---

## How It Fits

- **[fleet-topology](https://github.com/SuperInstance/fleet-topology)** — network model (this)
- **[fleet-constraint](https://github.com/SuperInstance/fleet-constraint)** — safety constraint runtime
- **[fleet-coordinate](https://github.com/SuperInstance/fleet-coordinate)** — spatial coordination on hex lattices
- **[beacon-protocol](https://github.com/SuperInstance/beacon-protocol)** — discovery and registry
- **[cocapn-glue-core](https://github.com/SuperInstance/cocapn-glue-core)** — wire protocol

---

## License

MIT
