pub mod legal;
pub mod pricing;

use {
    legal::Legal,
    pricing::Pricing,
    serde::Deserialize
};

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[allow(unused)]
    pub name: String,
    #[allow(unused)]
    pub version: Vec<usize>,
    #[allow(unused)]
    pub legal: Legal,
    #[allow(unused)]
    pub pricing: Option<Pricing>
}
