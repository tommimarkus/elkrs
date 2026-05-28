use elkrs_core::diagnostic::Diagnostic;
use elkrs_core::layout::LayoutError;
use elkrs_core::options::{Algorithm, CoreOption, HierarchyHandling, Properties};

const UNSUPPORTED_OPTION_CODE: &str = "ELKRS_LAYERED_UNSUPPORTED_OPTION";

pub(crate) fn validate_options(properties: &Properties) -> Result<Vec<Diagnostic>, LayoutError> {
    match properties.algorithm() {
        Some(Algorithm::Layered) | None => {}
        Some(Algorithm::Other(algorithm)) => {
            return Err(LayoutError::UnsupportedAlgorithm(algorithm));
        }
    }

    let mut diagnostics = Vec::new();
    if matches!(
        properties.hierarchy_handling(),
        HierarchyHandling::SeparateChildren
    ) {
        diagnostics.push(Diagnostic::warning(
            UNSUPPORTED_OPTION_CODE,
            "hierarchy handling SeparateChildren is recognized but not implemented by elkrs-layered yet; laying out children with the current graph",
        ));
    }
    if properties.get(CoreOption::SpacingEdgeNode).is_some() {
        diagnostics.push(Diagnostic::warning(
            UNSUPPORTED_OPTION_CODE,
            "edge-node spacing is recognized but not applied by elkrs-layered edge routing yet",
        ));
    }
    if properties.get(CoreOption::SpacingEdgeEdge).is_some() {
        diagnostics.push(Diagnostic::warning(
            UNSUPPORTED_OPTION_CODE,
            "edge-edge spacing is recognized but not applied by elkrs-layered edge routing yet",
        ));
    }

    Ok(diagnostics)
}
