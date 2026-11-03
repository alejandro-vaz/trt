use {
    clap::Parser,
    std::path::PathBuf
};

#[derive(Parser)]
#[command(
    name = env!("CARGO_BIN_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    disable_help_flag = true,
    disable_version_flag = true
)]
pub enum Command {
    #[command(about = "Build an injection")]
    Build {
        #[arg(default_value = "./")]
        directory: PathBuf
    },
    #[command(about = "Show the installed TRT version")]
    Version
}
