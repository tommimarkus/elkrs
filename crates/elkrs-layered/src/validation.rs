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
    validate_non_negative_spacing(properties, CoreOption::SpacingEdgeNode, "edge-node spacing")?;
    validate_non_negative_spacing(properties, CoreOption::SpacingEdgeEdge, "edge-edge spacing")?;

    Ok(diagnostics)
}

fn validate_non_negative_spacing(
    properties: &Properties,
    option: CoreOption,
    name: &str,
) -> Result<(), LayoutError> {
    let Some(spacing) = spacing_value(properties, option) else {
        return Ok(());
    };
    if spacing.is_finite() && spacing >= 0.0 {
        return Ok(());
    }

    Err(LayoutError::InvalidOption(format!(
        "{name} must be finite and non-negative"
    )))
}

fn spacing_value(properties: &Properties, option: CoreOption) -> Option<f64> {
    match option {
        CoreOption::SpacingEdgeNode => Some(properties.spacing_edge_node()),
        CoreOption::SpacingEdgeEdge => Some(properties.spacing_edge_edge()),
        _ => None,
    }
}
