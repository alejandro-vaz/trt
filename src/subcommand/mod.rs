pub mod action;

use {
    super::information::{
        PKG_AUTHORS,
        PKG_DESCRIPTION,
        PKG_NAME,
        PKG_VERSION
    },
    action::Action,
    clap::Parser
};

#[derive(Parser)]
#[command(
    name = PKG_NAME,
    version = PKG_VERSION,
    about = PKG_DESCRIPTION,
    disable_help_flag = true,
    disable_version_flag = true,
    subcommand_required = true,
    propagate_version = true,
    long_version = PKG_VERSION,
    author = PKG_AUTHORS,
    disable_help_subcommand = true
)]
pub enum Subcommand {
    #[command(about = "Build an injection")]
    Build,
    #[command(name = "@", about = "Manage this TRT installation")]
    Installation {
        #[command(subcommand)]
        action: Action
    }
}
