use std::{fmt::format, path::PathBuf, str::EncodeUtf16};

use nu_plugin::{
    EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand, serve_plugin,
};
use nu_protocol::{LabeledError, LsConfig, PipelineData, Signature, SyntaxShape, Type, Value};

mod fs;
mod parse;
mod print;

struct SPlugin;

impl Plugin for SPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(S)]
    }
}

struct S;

impl PluginCommand for S {
    type Plugin = SPlugin;
    fn name(&self) -> &str {
        "s"
    }
    fn description(&self) -> &str {
        "see and select items by index"
    }
    fn extra_description(&self) -> &str {
        "enter the index of an item to pipe it's path to stdout:

s 20 | mv $in ./tmp"
    }
    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .optional(
                "index",
                SyntaxShape::OneOf(vec![
                    SyntaxShape::Int,
                    SyntaxShape::Range,
                    SyntaxShape::List(Box::new(SyntaxShape::OneOf(vec![
                        SyntaxShape::Int,
                        SyntaxShape::Range,
                    ]))),
                ]),
                "index of item(s) you want to select",
            )
            .named(
                "recursive",
                SyntaxShape::Int,
                "recurse into directories this far",
                Some('r'),
            )
            .named(
                "dir-first",
                SyntaxShape::Int,
                "show directories before files",
                Some('d'),
            )
    }
    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, LabeledError> {
        let span = call.head;
        let metadata = input.metadata();
        let args = match parse::SArgs::parse_call(call, engine) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        // let env = parse::get_env(engine)
        // let cfg = parse::merge(args, env);
        let cwd = match engine.get_current_dir() {
            Ok(s) => PathBuf::from(s),
            Err(_) => match dirs::home_dir() {
                Some(pb) => pb,
                None => {
                    return Err(LabeledError::new("home dir not found"));
                }
            },
        };
        let enum_list = fs::DirList::new(cwd, &args);

        if let Some(v) = args.sel {
            for s in v {
                println!("{}", enum_list.nth(s).display())
            }
        }
        if args.print {
            print::enum_list(&enum_list, &args);
        }

        Ok(PipelineData::Empty)
    }
}

fn main() {
    serve_plugin(&SPlugin, MsgPackSerializer);
}
