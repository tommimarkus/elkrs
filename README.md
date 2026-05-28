# elkrs

elkrs is a Rust port of Eclipse Layout Kernel concepts and behavior.

The first target is library-first strict clean-room ELK Layered behavior aligned with ELK v0.11.0.

Downstream users adapt their own graph contracts.

Current scope:

- Rust graph, geometry, options, diagnostics, and layout error/report types
- Initial `elkrs-layered` layout API with `LayeredLayout` and `LayoutAlgorithm`
- Deterministic, structurally valid layout for simple directed graphs, port endpoints, and basic compound child nodes

Not current scope:

- Dediren adapter
- CLI-first runtime
- Full ELK algorithm coverage
- Pixel-perfect Java coordinate parity
- Copying Eclipse ELK implementation source
