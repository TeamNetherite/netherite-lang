use codespan_reporting::diagnostic::{Label, LabelStyle, Severity};
use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use std::ops::Range;
use std::process::exit;

/// Stores basic `codespan_reporting` structs for reporting errors.
pub struct ReporterState<'f> {
    pub writer: StandardStream,
    pub config: Config,
    pub files: SimpleFiles<&'f str, &'f str>,
    pub cli_command_id: Option<usize>,
}

impl<'f> ReporterState<'f> {
    pub fn cli(command: &'f str) -> Self {
        let mut files = SimpleFiles::new();
        let cmd_id = files.add("command", command);
        Self {
            writer: StandardStream::stdout(ColorChoice::Auto),
            files,
            cli_command_id: Some(cmd_id),
            config: Config::default(),
        }
    }

    pub fn emit_global_error(&self, msg: &str) {
        term::emit(
            &mut self.writer.lock(),
            &self.config,
            &self.files,
            &Diagnostic::error().with_message(msg),
        )
        .expect("emit_global_diagnostic() failed");
    }

    pub fn emit_cli(&self, msg: &str, err_code: &str, span: Range<usize>, severity: Severity) {
        term::emit(
            &mut self.writer.lock(),
            &self.config,
            &self.files,
            &Diagnostic::new(severity)
                .with_message(msg)
                .with_code(err_code)
                .with_labels(vec![Label::new(
                    LabelStyle::Primary,
                    self.cli_command_id.unwrap(),
                    span,
                )]),
        )
        .expect("emit_global_diagnostic() failed")
    }

    pub fn emit_cli_err(&self, msg: &str, err_code: &str, span: Range<usize>) {
        self.emit_cli(msg, err_code, span, Severity::Error)
    }

    pub fn emit_cli_fatal(&self, msg: &str, err_code: &str, span: Range<usize>) -> ! {
        self.emit_cli_err(msg, err_code, span);
        exit(1)
    }
}

impl Default for ReporterState<'_> {
    fn default() -> Self {
        Self {
            writer: StandardStream::stderr(ColorChoice::Always),
            config: Config::default(),
            files: SimpleFiles::new(),
            cli_command_id: None,
        }
    }
}

pub trait Reporter<'source> {
    fn emit_diagnostic(
        &self,
        reporter: &ReporterState,
        files: &SimpleFiles<&str, &str>,
        file_id: usize,
    ) {
        term::emit(
            &mut reporter.writer.lock(),
            &reporter.config,
            files,
            &self.build_diagnostic(file_id),
        )
        .expect("emit_diagnostic() failed")
    }

    fn build_diagnostic(&self, file_id: usize) -> Diagnostic<usize>;
}
