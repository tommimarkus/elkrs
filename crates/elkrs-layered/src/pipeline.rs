use elkrs_core::diagnostic::Diagnostic;
use elkrs_core::layout::LayoutError;

use crate::internal::LGraph;

pub(crate) struct LayeredContext {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) trace: Vec<&'static str>,
}

impl LayeredContext {
    pub(crate) fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            trace: Vec::new(),
        }
    }
}

pub(crate) trait LayeredProcessor {
    fn name(&self) -> &'static str;
    fn run(&self, graph: &mut LGraph, context: &mut LayeredContext) -> Result<(), LayoutError>;
}

pub(crate) struct LayeredPipeline {
    processors: Vec<Box<dyn LayeredProcessor>>,
}

impl LayeredPipeline {
    pub(crate) fn new(processors: Vec<Box<dyn LayeredProcessor>>) -> Self {
        Self { processors }
    }

    pub(crate) fn run(&self, graph: &mut LGraph) -> Result<LayeredContext, LayoutError> {
        let mut context = LayeredContext::new();
        for processor in &self.processors {
            context.trace.push(processor.name());
            processor
                .run(graph, &mut context)
                .map_err(|error| match error {
                    LayoutError::PhaseFailed { .. } => error,
                    other => LayoutError::PhaseFailed {
                        phase: processor.name(),
                        message: other.to_string(),
                    },
                })?;
        }
        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Named(&'static str);

    impl LayeredProcessor for Named {
        fn name(&self) -> &'static str {
            self.0
        }

        fn run(
            &self,
            _graph: &mut LGraph,
            _context: &mut LayeredContext,
        ) -> Result<(), LayoutError> {
            Ok(())
        }
    }

    #[test]
    fn pipeline_runs_processors_in_order() {
        let pipeline = LayeredPipeline::new(vec![
            Box::new(Named("cycle-breaking")),
            Box::new(Named("layer-assignment")),
            Box::new(Named("edge-routing")),
        ]);
        let mut graph = LGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        };

        let context = pipeline.run(&mut graph).unwrap();

        assert_eq!(
            context.trace,
            vec!["cycle-breaking", "layer-assignment", "edge-routing"]
        );
    }
}
