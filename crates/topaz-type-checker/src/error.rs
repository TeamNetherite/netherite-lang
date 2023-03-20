use codespan_reporting::diagnostic::Diagnostic;
use thiserror::Error;
use topaz_report::Reporter;

#[derive(Error, Copy, Clone, Debug, PartialEq, Eq)]
pub enum StaticAnalysisError {}

impl<'source> Reporter<'source> for StaticAnalysisError {
    fn build_diagnostic(&self, _file_id: usize) -> Diagnostic<usize> {
        todo!()
    }
}
