#![allow(missing_docs)]
#![deny(unused, deprecated, unreachable_code, nonstandard_style)]
#![forbid(
    missing_debug_implementations,
    dropping_copy_types,
    dropping_references,
    elided_lifetimes_in_paths,
    explicit_builtin_cfgs_in_flags,
    forgetting_copy_types,
    forgetting_references,
    impl_trait_redundant_captures,
    invalid_doc_attributes,
    irrefutable_let_patterns,
    macro_use_extern_crate,
    missing_copy_implementations,
    mutable_transmutes,
    noop_method_call,
    opaque_hidden_inferred_bound,
    overflowing_literals,
    redundant_imports,
    single_use_lifetimes,
    trivial_casts,
    unknown_lints,
    arithmetic_overflow,
    type_alias_bounds
)]

mod commands;
mod manifest;

use {
    anyhow::Result,
    clap::Parser,
    commands::Command,
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
    };
}
