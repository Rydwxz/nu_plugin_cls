use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{LabeledError, Range, Value};
use std::path::PathBuf;

pub struct SArgs {
    pub sel: Option<Vec<i64>>,
    pub cmd: Option<SCmd>,
    pub pth: Option<PathBuf>,
    pub print: bool,
    pub flags: bool,
    pub recursive: i64,
    pub max_height: i64,
}

fn parse_config(eng: &EngineInterface) -> Option<SArgs> {
    if let Ok(Some(cfg)) = eng.get_plugin_config() {
        Some(
            SArgs::default()
                .recursive(cfg.get_data_by_key("recursive"))
                .max_height(cfg.get_data_by_key("max_height")),
        )
    } else {
        None
    }
}
fn parse_args(eng: &EngineInterface, call: &EvaluatedCall) -> SArgs {
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
    let mut flags = false;
    let recursive = match call.get_flag::<i64>("recursive")? {
        Some(i) => {
            flags = true;
            i
        }
        None => 0,
    };
    let print = if let None = sel { true } else { false };
    SArgs {
        sel,
        cmd: None,
        pth: None,
        print,
        flags,
        recursive,
        max_height: 42,
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

impl SArgs {
    fn default() -> Self {
        Self {
            sel: None,
            cmd: None,
            pth: None,
            print: false,
            flags: false,
            recursive: 0,
            max_height: 58,
        }
    }
    fn merge(self, other: Self) -> Self {
        Self {
            sel: if let Some(new) = other.sel {
                Some(new)
            } else {
                self.sel
            },
            cmd: if let Some(new) = other.cmd {
                Some(new)
            } else {
                self.cmd
            },
        }
    }
    fn mrg<T>(old: T, new: T) -> T {
        if let Some(new) = new { new } else { old }
    }
    fn merge_option<T>(old: Option<T>, new: Option<T>) -> Option<T> {
        if let Some(new) = new { Some(new) } else { old }
    }
    fn merge_bool<T>(old: bool, new: bool) -> bool {
        if let Some(new) = new { Some(new) } else { old }
    }
    fn recursive(mut self, flag: Option<Value>) -> Self {
        if let Some(v) = flag {
            if let Ok(i) = v.as_int() {
                if i > 0 {
                    self.recursive = i;
                }
            }
        }
        self
    }
    fn max_height(mut self, flag: Option<Value>) -> Self {
        if let Some(v) = flag {
            if let Ok(i) = v.as_int() {
                if i > 0 {
                    self.max_height = i;
                }
            }
        }
        self
    }
}
pub enum SCmd {}
