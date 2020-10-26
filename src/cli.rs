use anyhow::Result;
use serde::Serialize;
use structopt::StructOpt;

use crate::command;

#[derive(Debug, Serialize, StructOpt)]
#[structopt(name = "Rover", about = "✨🤖🐶 the new CLI for apollo")]
pub struct Rover {
    #[structopt(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Serialize, StructOpt)]
pub enum Command {
    ///  ⚙️  Manage configuration
    Config(command::Config),
    ///  🧱 Fetch a schema
    Schema(command::Schema),
    ///  Work with federated services
    Service(command::Service),
}

impl Rover {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Config(config) => config.run(),
            Command::Schema(schema) => schema.run(),
            Command::Service(service) => service.run(),
        }
    }
}
