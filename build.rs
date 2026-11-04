#![doc = include_str!("./README.md")]

use {
    anyhow::Result,
    built::write_built_file
};

fn main() -> Result<()> {
    write_built_file()?;
    Result::Ok(())
}
