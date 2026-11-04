#![doc = include_str!("./README.md")]
#[cfg(all(debug_assertions, not(rust_analyzer), not(doc)))]
compile_error!("cannot build TRT on debug mode");

mod command;
mod information;
mod manifest;
mod version;

use {
    anyhow::Result,
    clap::Parser,
    command::Command,
    information::{
        CFG_FAMILY,
        CFG_OS,
        CFG_TARGET_ARCH,
        PKG_AUTHORS,
        PKG_DESCRIPTION,
        PKG_NAME
    },
    manifest::Manifest,
    std::{
        env::current_dir,
        fs::read_to_string
    },
    toml::from_str,
    version::VERSION
};

fn main() -> Result<()> {
    match Command::parse() {
        Command::Build => {
            let manifest = from_str::<Manifest>(
                read_to_string(current_dir().unwrap().join("manifest.toml"))?.as_str()
            )?;
            dbg!(&manifest);
            todo!()
        }
        Command::Version => {
            println!(
                "{PKG_NAME} {VERSION}-{CFG_OS}-{CFG_FAMILY}-{CFG_TARGET_ARCH}
-- {PKG_DESCRIPTION}

(c) {PKG_AUTHORS}"
            );
            Result::Ok(())
        }
    }
}
