use {
    super::information::{
        PKG_AUTHORS,
        PKG_DESCRIPTION,
        PKG_NAME,
        PKG_VERSION
    },
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
    author = PKG_AUTHORS
)]
pub enum Command {
    #[command(about = "Build an injection")]
    Build,
    #[command(about = "Show the installed TRT version")]
    Version,
    #[command(about = "Show the TRT license")]
    License
}
