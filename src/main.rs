use std::{fmt::format, str::EncodeUtf16};

use nu_plugin::{
    EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand, serve_plugin,
};
use nu_protocol::{LabeledError, LsConfig, PipelineData, Signature, Type, Value};

mod cli;

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
        "seel"
    }
    fn description(&self) -> &str {
        "grab list entries by index"
    }
    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
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
        let cli_args = cli::CliArgs::parse_call(call);
        let cwd = engine.get_current_dir();

        println!("Hello Nush");

        Ok(PipelineData::Empty)
    }
}

fn main() {
    serve_plugin(&SeelPlugin, MsgPackSerializer);
}

fn show_dir() {}

fn select_string() {}
