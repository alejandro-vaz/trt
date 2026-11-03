pub mod command;

use {
    clap::Parser,
    command::Command
};

#[derive(Parser)]
#[command(
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command
}
