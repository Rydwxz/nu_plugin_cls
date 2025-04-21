use walkdir::DirEntry;

use std::{fmt::format, path::PathBuf, str::EncodeUtf16};

use nu_plugin::{
    EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand, serve_plugin,
};
use nu_protocol::{LabeledError, LsConfig, PipelineData, Signature, SyntaxShape, Type, Value};

mod fs;
mod parse;
mod print;

struct SeelPlugin;

impl Plugin for SeelPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Seel)]
    }
}

struct Seel;

impl PluginCommand for Seel {
    type Plugin = SeelPlugin;
    fn name(&self) -> &str {
        "cls"
    }
    fn description(&self) -> &str {
        "visually select directory items"
    }
    fn extra_description(&self) -> &str {
        "enter the index of an item to pipe it's path to stdout:

seel 20 | mv $in ./tmp"
    }
    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            // .input_output_types(vec![
            //     (Type::Nothing, Type::Nothing),
            //     (Type::Int, Type::Nothing),
            // ])
            .optional(
                "index",
                SyntaxShape::Int,
                "index of one item you want to select",
            )
            .optional(
                "range",
                SyntaxShape::Range,
                "range of indexes you want to select",
            )
            .optional(
                "list",
                SyntaxShape::List(Box::new(SyntaxShape::Any)),
                "list of ranges and/or indexes you want to select",
            )
            .named(
                "recursive",
                SyntaxShape::Int,
                "recurse into directories this far",
                Some('r'),
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
        let args = match parse::SeelArgs::parse_call(call) {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        let cwd = match engine.get_current_dir() {
            Ok(s) => PathBuf::from(s),
            Err(_) => match dirs::home_dir() {
                Some(pb) => pb,
                None => {
                    return Err(LabeledError::new("home dir not found"));
                }
            },
        };
        let enum_list = fs::walk(cwd, args);

        print::enum_list(enum_list);

        Ok(PipelineData::Empty)
    }
}

fn main() {
    serve_plugin(&SeelPlugin, MsgPackSerializer);
}
