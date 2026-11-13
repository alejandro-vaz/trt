#![doc = include_str!("../.github/README.md")]
#![feature(const_bool, const_trait_impl)]
#[cfg(all(debug_assertions, not(rust_analyzer)))]
compile_error!("cannot build TRT on debug mode");
#[cfg(doc)]
compile_error!("not meant to compile for documentation");

mod information {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
mod manifest;
mod subcommand;
mod version;

use {
    anyhow::Result,
    clap::Parser,
    information::{
        CFG_FAMILY,
        CFG_OS,
        CFG_TARGET_ARCH,
        PKG_AUTHORS,
        PKG_DESCRIPTION,
        PKG_NAME
    },
    manifest::Manifest,
    reqwest::blocking::get,
    self_replace::self_replace,
    std::{
        env::current_dir,
        fs::read,
        io::copy
    },
    subcommand::{
        Subcommand,
        action::Action
    },
    tempfile::NamedTempFile,
    termimad::MadSkin,
    toml::from_slice,
    version::Version
};

fn main() -> Result<()> {
    match Subcommand::parse() {
        Subcommand::Build => {
            let manifest =
                from_slice::<Manifest>(read(current_dir()?.join("manifest.toml"))?.as_slice())?;
            dbg!(&manifest);
        }
        Subcommand::Installation {
            action
        } => match action {
            Action::License => MadSkin::default_dark().print_text(include_str!("../LICENSE.md")),
            Action::Version => println!(
                "{PKG_NAME} {}-{CFG_OS}-{CFG_FAMILY}-{CFG_TARGET_ARCH}
-- {PKG_DESCRIPTION}

(c) {PKG_AUTHORS}",
                Version::COMPILER
                    .as_ref()
                    .map(ToString::to_string)
                    .as_deref()
                    .unwrap_or("(experimental)")
            ),
            Action::Update => {
                let mut handle = NamedTempFile::new()?;
                copy(
                    &mut get("https://github.com/trtlang/trt/releases/latest/download/trt")?
                        .error_for_status()?,
                    handle.as_file_mut()
                )?;
                handle.as_file().sync_all()?;
                self_replace(handle)?;
                println!("Successfully updated TRT.");
            }
        }
    }
    Result::Ok(())
}
