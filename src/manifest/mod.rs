pub mod injection;

use {
    injection::Injection,
    serde::Deserialize
};

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[allow(unused)]
    injection: Injection
}
