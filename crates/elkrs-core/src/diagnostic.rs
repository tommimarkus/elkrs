#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub element_id: Option<String>,
}

impl Diagnostic {
    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Warning,
            message: message.into(),
            element_id: None,
        }
    }

    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            element_id: None,
        }
    }

    pub fn with_element(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attaches_element_context() {
        let diagnostic = Diagnostic::warning("W_TEST", "message").with_element("node-a");

        assert_eq!(diagnostic.element_id.as_deref(), Some("node-a"));
    }
}
