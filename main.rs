#![allow(missing_docs)]
#![deny(unused, deprecated, unreachable_code)]
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
    nonstandard_style,
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

mod arguments;

use {
    arguments::Arguments,
    clap::Parser
};

fn main() { let _arguments = Arguments::parse(); }
