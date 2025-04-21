use nu_plugin::EvaluatedCall;
use nu_protocol::LabeledError;
use std::path::PathBuf;

pub struct CliArgs {
    sel: Option<Vec<i32>>,
    cmd: Option<SeelCmd>,
    pth: Option<PathBuf>,
}

impl CliArgs {
    pub fn parse_call(call: &EvaluatedCall) -> Result<Self, LabeledError> {
        Ok(Self {
            sel: None,
            cmd: None,
            pth: None,
        })
    }
}

pub enum SeelCmd {}
