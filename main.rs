#![doc = include_str!("./README.md")]

mod command;
mod manifest;

use {
    anyhow::Result,
    clap::Parser,
    command::Command,
    manifest::Manifest,
    std::fs::read_to_string,
    toml::from_str
};

fn main() -> Result<()> {
    match Command::parse() {
        Command::Build {
            directory
        } => {
            let manifest =
                from_str::<Manifest>(read_to_string(directory.join("manifest.toml"))?.as_str())?;
            dbg!(&manifest);
            todo!()
        }
        Command::Version => {
            println!("TRT v{}", env!("CARGO_PKG_VERSION"));
            Result::Ok(())
        }
    }
}
