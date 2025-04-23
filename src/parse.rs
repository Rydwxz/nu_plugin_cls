use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{LabeledError, Range, Value, engine::EngineState};
use std::path::PathBuf;

pub struct SArgs {
    pub sel: Option<Vec<i64>>,
    pub cmd: Option<SCmd>,
    pub pth: Option<PathBuf>,
    pub recursive: i64,
    pub print: bool,
}

impl SArgs {
    pub fn parse_call(call: &EvaluatedCall, eng: &EngineInterface) -> Result<Self, LabeledError> {
        let sel = if let Some(first) = call.nth(0) {
            if let Ok(i) = first.as_int() {
                Some(vec![i])
            } else if let Ok(r) = first.as_range() {
                Some(flatten_range(r, call, eng))
            } else if let Ok(l) = first.as_list() {
                Some(l.into_iter().fold(vec![], |mut acc, v| {
                    match v {
                        Value::Int { val, .. } => acc.push(*val),
                        Value::Range { val, .. } => flatten_range(**val, call, eng)
                            .iter()
                            .for_each(|i| acc.push(*i)),
                        _ => {}
                    };
                    acc
                }))
            } else {
                None
            }
        } else {
            None
        };
        let print = if let None = sel { true } else { false };
        Ok(Self {
            sel,
            cmd: None,
            pth: None,
            recursive: match call.get_flag::<i64>("recursive")? {
                Some(i) => i + 1,
                None => 1,
            },
            print,
        })
    }
}

fn flatten_range(r: Range, call: &EvaluatedCall, eng: &EngineInterface) -> Vec<i64> {
    r.into_range_iter(call.head, eng.signals().clone())
        .fold(vec![], |mut acc, i| {
            if let Ok(i) = i.as_int() {
                acc.push(i)
            };
            acc
        })
}

pub enum SCmd {}
