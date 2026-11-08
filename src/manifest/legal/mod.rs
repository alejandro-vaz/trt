pub mod license;

use {
    license::License,
    serde::Deserialize
};

#[derive(Debug, Deserialize)]
pub struct Legal {
    #[allow(unused)]
    pub license: License
}
