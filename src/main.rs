#![doc = include_str!("../README.md")]
#[cfg(all(debug_assertions, not(rust_analyzer)))]
compile_error!("cannot build TRT on debug mode");
#[cfg(doc)]
compile_error!("not meant to compile for documentation");

mod action;
mod information {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
mod manifest;

use {
    action::Action,
    anyhow::Result,
    clap::Parser,
    information::{
        CFG_FAMILY,
        CFG_OS,
        CFG_TARGET_ARCH,
        GIT_DIRTY,
        GIT_VERSION,
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
    tempfile::NamedTempFile,
    termimad::MadSkin,
    toml::from_slice
};

fn main() -> Result<()> {
    match Action::parse() {
        Action::Build => {
            let manifest =
                from_slice::<Manifest>(read(current_dir()?.join("manifest.toml"))?.as_slice())?;
            dbg!(&manifest);
        }
        Action::Version => println!(
            "{PKG_NAME} {}-{CFG_OS}-{CFG_FAMILY}-{CFG_TARGET_ARCH}
-- {PKG_DESCRIPTION}

(c) {PKG_AUTHORS}",
            if unsafe { GIT_DIRTY.unwrap_unchecked() } {
                "(experimental)"
            } else {
                unsafe { GIT_VERSION.unwrap_unchecked() }
            }
        ),
        Action::License => MadSkin::default_dark().print_text(include_str!("../LICENSE.md")),
        Action::Update => {
            let mut handle = NamedTempFile::new()?;
            let file = handle.as_file_mut();
            copy(
                &mut get("https://github.com/trtlang/trt/releases/latest/download/trt")?
                    .error_for_status()?,
                file
            )?;
            file.sync_all()?;
            self_replace(handle)?;
            println!("Successfully updated TRT.");
        }
    }
    Result::Ok(())
}
