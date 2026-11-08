use {
    chrono::{
        DateTime,
        Utc
    },
    core::time::Duration,
    serde::Deserialize
};

#[derive(Debug, Deserialize)]
pub struct Period {
    #[allow(unused)]
    pub anchor: Option<DateTime<Utc>>,
    #[allow(unused)]
    pub length: Duration
}
