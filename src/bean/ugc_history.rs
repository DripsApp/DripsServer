use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UGCHistoryBean {
    pub id: Option<i32>,
    pub ref_id: i32,
    pub title: Option<String>,
    pub content: String,
    pub created: i64,
}
rbatis::crud!(UGCHistoryBean {}, "ugc_histories");
