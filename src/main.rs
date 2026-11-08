#![doc = include_str!("../README.md")]
#[cfg(all(debug_assertions, not(rust_analyzer), not(doc)))]
compile_error!("cannot build TRT on debug mode");

mod action;
mod information;
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
        fs::read_to_string,
        io::copy
    },
    tempfile::NamedTempFile,
    termimad::MadSkin,
    toml::from_str
};

fn main() -> Result<()> {
    match Action::parse() {
        Action::Build => {
            let manifest = from_str::<Manifest>(
                read_to_string(current_dir()?.join("manifest.toml"))?.as_str()
            )?;
            dbg!(&manifest);
            todo!()
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
            let mut response = get("https://github.com/trtlang/trt/releases/latest/download/trt")?
                .error_for_status()?;
            let file = handle.as_file_mut();
            copy(&mut response, file)?;
            file.sync_all()?;
            self_replace(handle)?;
            println!("Successfully updated TRT.");
        }
    }
    Result::Ok(())
}
