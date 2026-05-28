# elkrs

elkrs is a Rust port of Eclipse Layout Kernel concepts and behavior.

The first target is library-first strict clean-room ELK Layered behavior aligned with ELK v0.11.0.

Downstream users adapt their own graph contracts.

Current scope:

- Rust graph, geometry, options, diagnostics, and layout error/report types
- Initial `elkrs-layered` layout API with `LayeredLayout` and `LayoutAlgorithm`
- Initial `elkrs-json` import/export API for a narrow ELK-style JSON subset
- Opt-in Java ELK black-box comparison harness via `ELKRS_JAVA_ELK_COMMAND`
- Typed direction, spacing, algorithm, routing, and hierarchy options for the initial supported subset
- Diagnostics for recognized but unsupported `elkrs-layered` options
- Deterministic, structurally valid layout for simple directed graphs, port-aware edge endpoints, and basic compound child nodes
- Stable layer normalization across equivalent node insertion orders
- Basic barycenter-style crossing minimization for adjacent layers
- Basic compound child placement inside parent bounds
- Simple orthogonal edge detours around the first unrelated node obstacle
- Import-time validation for duplicate node IDs, missing endpoints, and self-loop edges
- Layout output writes child node coordinates as absolute graph coordinates

Not current scope:

- Dediren adapter
- CLI-first runtime
- Full ELK algorithm coverage
- Pixel-perfect Java coordinate parity
- Copying Eclipse ELK implementation source
