use {
    clap::Subcommand,
    std::path::PathBuf
};

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Build the current project")]
    Build {
        #[arg(default_value = "./")]
        directory: PathBuf
    }
}
