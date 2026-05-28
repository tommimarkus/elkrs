use thiserror::Error;

use crate::diagnostic::Diagnostic;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LayoutError {
    #[error("unsupported layout algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("missing endpoint: {0}")]
    MissingEndpoint(String),

    #[error("invalid hierarchy: {0}")]
    InvalidHierarchy(String),

    #[error("layout phase {phase} failed: {message}")]
    PhaseFailed {
        phase: &'static str,
        message: String,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutReport {
    pub diagnostics: Vec<Diagnostic>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_failed_display_includes_phase_name() {
        let error = LayoutError::PhaseFailed {
            phase: "crossing-minimization",
            message: "failed".to_string(),
        };

        assert!(error.to_string().contains("crossing-minimization"));
    }
}
