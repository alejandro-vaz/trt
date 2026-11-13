use clap::Subcommand;

#[derive(Subcommand)]
#[command(disable_help_subcommand = true)]
pub enum Action {
    #[command(about = "Show the TRT license")]
    License,
    #[command(about = "Update TRT to the latest version")]
    Update,
    #[command(about = "Show the installed TRT version")]
    Version
}
