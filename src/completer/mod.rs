use std::result::Result;

use rustyline::{
    completion::{Candidate, Completer},
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Context, Helper,
};

use crate::cli::COMMANDS;

#[derive(Debug, Clone)]
pub struct CommandCandidate {
    display: String,
    replacement: String,
}

impl Candidate for CommandCandidate {
    fn display(&self) -> &str {
        &self.display
    }

    fn replacement(&self) -> &str {
        &self.replacement
    }
}

pub struct ClashShellHelper {
    commands: Vec<String>,
}

impl ClashShellHelper {
    pub fn new() -> Self {
        Self {
            commands: COMMANDS.to_vec(),
        }
    }
}

impl Completer for ClashShellHelper {
    type Candidate = CommandCandidate;

    fn complete(
        &self,
        line: &str,
        _: usize,
        _: &Context<'_>,
    ) -> Result<(usize, Vec<CommandCandidate>), ReadlineError> {
        let matches: Vec<Self::Candidate> = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(line))
            .map(|cmd| CommandCandidate {
                display: cmd.clone(),
                replacement: cmd.clone(),
            })
            .collect();
        Ok((0, matches))
    }
}

impl Highlighter for ClashShellHelper {}

impl Hinter for ClashShellHelper {
    type Hint = String;

    fn hint(&self, _: &str, _: usize, _: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Validator for ClashShellHelper {}

impl Helper for ClashShellHelper {}
