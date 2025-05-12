use nu_plugin::{EngineInterface, EvaluatedCall};
use nu_protocol::{LabeledError, Range, Value};
use std::path::PathBuf;

pub struct SArgs {
    pub recursive: Option<i64>,
    pub max_height: Option<i64>,
    // these should never be set by configuration: only at runtime
    pub sel: Option<Vec<i64>>,
    pub cmd: Option<SCmd>,
    pub pth: Option<PathBuf>,
    pub print: bool,
    pub flags: bool,
}

pub fn parse_config(eng: &EngineInterface) -> Option<SArgs> {
    if let Ok(Some(cfg)) = eng.get_plugin_config() {
        Some(SArgs {
            recursive: if let Some(val) = cfg.get_data_by_key("recursive") {
                if let Ok(i) = val.as_int() {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            },
            max_height: if let Some(val) = cfg.get_data_by_key("max_height") {
                if let Ok(i) = val.as_int() {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            },
            ..SArgs::default()
        })
    } else {
        None
    }
}
pub fn parse_args(eng: &EngineInterface, call: &EvaluatedCall) -> SArgs {
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
    let recursive = if let Ok(o) = call.get_flag::<i64>("recursive") {
        if let Some(i) = o {
            flags = true;
            Some(i)
        } else {
            None
        }
    } else {
        None
    };
    let max_height = None;
    let print = if let None = sel { true } else { false };
    SArgs {
        sel,
        cmd: None,
        pth: None,
        print,
        flags,
        recursive,
        max_height,
    }
}

fn flatten_range(r: Range, call: &EvaluatedCall, eng: &EngineInterface) -> Vec<i64> {
    r.into_range_iter(call.head, eng.signals().clone())
        .fold(vec![], |mut acc, v| {
            if let Ok(i) = v.as_int() {
                acc.push(i)
            };
            acc
        })
}

impl SArgs {
    fn default() -> Self {
        Self {
            recursive: Some(0),
            max_height: Some(58),
            sel: None,
            cmd: None,
            pth: None,
            print: false,
            flags: false,
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
            pth: if let Some(new) = other.pth {
                Some(new)
            } else {
                self.pth
            },
            print: other.print, // even if cfg sets these,
            flags: other.flags, // args will overwrite afterwards
            recursive: if let Some(new) = other.recursive {
                Some(new)
            } else {
                self.recursive
            },
            max_height: if let Some(new) = other.max_height {
                Some(new)
            } else {
                self.max_height
            },
        }
    }
    pub fn recursive(&self) -> i64 {
        match self.recursive {
            Some(i) => i,
            None => 0, // unreachable
        }
    }
    // fn recursive(mut self, flag: Option<Value>) -> Self {
    //     if let Some(v) = flag {
    //         if let Ok(i) = v.as_int() {
    //             if i > 0 {
    //                 self.recursive = i;
    //             }
    //         }
    //     }
    //     self
    // }
    // fn max_height(mut self, flag: Option<Value>) -> Self {
    //     if let Some(v) = flag {
    //         if let Ok(i) = v.as_int() {
    //             if i > 0 {
    //                 self.max_height = i;
    //             }
    //         }
    //     }
    //     self
    // }
}
pub enum SCmd {}
