use elkrs_core::layout::LayoutError;

use crate::internal::LGraph;
use crate::pipeline::{LayeredContext, LayeredProcessor};

pub(crate) struct CrossingMinimization;

impl LayeredProcessor for CrossingMinimization {
    fn name(&self) -> &'static str {
        "crossing-minimization"
    }

    fn run(&self, _graph: &mut LGraph, _context: &mut LayeredContext) -> Result<(), LayoutError> {
        Ok(())
    }
}
