#![doc = include_str!(".github/README.md")]

use {
    built::write_built_file,
    std::io::Result
};

fn main() -> Result<()> { write_built_file() }
