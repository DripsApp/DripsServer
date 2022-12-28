use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pager {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}