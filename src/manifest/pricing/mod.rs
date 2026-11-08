pub mod period;

use {
    period::Period,
    serde::Deserialize
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Pricing {
    #[allow(unused)]
    Download { amount: f64 },
    #[allow(unused)]
    License { amount: f64, period: Period }
}
