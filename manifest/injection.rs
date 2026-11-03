use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Injection {
    #[allow(unused)]
    pub name: String
}
