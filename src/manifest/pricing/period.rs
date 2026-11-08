use {
    chrono::{
        DateTime,
        Utc
    },
    serde::Deserialize,
    core::time::Duration
};

#[derive(Debug, Deserialize)]
pub struct Period {
    #[allow(unused)]
    pub anchor: Option<DateTime<Utc>>,
    #[allow(unused)]
    pub length: Duration
}
