use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(version = env!("CARGO_PKG_RUST_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Arguments {}
