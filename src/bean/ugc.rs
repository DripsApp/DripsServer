use rbatis::impl_select_page;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UGCBean{
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: String,
    pub uid: String,
    pub likes: i32,
    pub comments: i32,
    pub created: i64,
    pub updated: i64,
    pub status: i32, // Status 0正常 -1隐藏 timestamp回收时间
}

rbatis::crud!(UGCBean {},"ugc");
impl_select_page!(UGCBean{select_page(uid:&str) => "`where uid == #{uid}`"},"ugc");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UGCInput{
    pub title: Option<String>,
    pub content: Option<String>
}