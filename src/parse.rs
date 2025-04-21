use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Value};
use std::path::PathBuf;

pub struct SeelArgs {
    pub sel: Option<Vec<i32>>,
    pub cmd: Option<SeelCmd>,
    pub pth: Option<PathBuf>,
    pub recursive: i32,
    pub print: bool,
}

impl SeelArgs {
    pub fn parse_call(call: &EvaluatedCall) -> Result<Self, LabeledError> {
        let first = call.positional[0];
        let sel = if let Ok(i) = first.as_int() { Some(vec![i]) } else if let Ok(r) = first.as_range() {
            r.into_range_iter(signals)
        }
        let print = if let None = sel { true } else { false };
        Ok(Self {
            sel,
            cmd: None,
            pth: None,
            recursive: match call.get_flag::<i32>("recursive")? {
                Some(i) => i + 1,
                None => 1,
            },
            print,
        })
    }
}

pub enum SeelCmd {}
