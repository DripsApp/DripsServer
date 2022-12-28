use rbatis::impl_select_page;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentBean {
    pub id: Option<i32>,
    pub ref_id: i32,
    pub ref_type: i32, //0帖子 1回复
    pub uid: String, 
    pub content: String,
    pub created: i64,
    pub status: i64 // Status 0正常 -1隐藏 timestamp回收时间
}
rbatis::crud!(CommentBean {}, "comments");
impl_select_page!(CommentBean{select_page(ref_type:&str,ref_id:&str) => "`where ref_type == #{ref_type} and ref_id == #{ref_id}`"},"comments");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UGCInput{
    pub ref_id: Option<i32>,
    pub ref_type: Option<i32>,
    pub content: Option<String>
}