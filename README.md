# elkrs

elkrs is a Rust port of Eclipse Layout Kernel concepts and behavior.

The first target is library-first strict clean-room ELK Layered behavior aligned with ELK v0.11.0.

Downstream users adapt their own graph contracts.

Current scope:

- Rust graph, geometry, options, diagnostics, and layout error/report types
- Initial `elkrs-layered` layout API with `LayeredLayout` and `LayoutAlgorithm`
- Typed direction, spacing, algorithm, routing, and hierarchy options for the initial supported subset
- Diagnostics for recognized but unsupported `elkrs-layered` options
- Deterministic, structurally valid layout for simple directed graphs, port-aware edge endpoints, and basic compound child nodes
- Import-time validation for duplicate node IDs, missing endpoints, and self-loop edges
- Layout output writes child node coordinates as absolute graph coordinates

Not current scope:

- Dediren adapter
- CLI-first runtime
- Full ELK algorithm coverage
- Pixel-perfect Java coordinate parity
- Copying Eclipse ELK implementation source
