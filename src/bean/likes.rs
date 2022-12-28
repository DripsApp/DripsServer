use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LikeBean{
    pub id:Option<i32>,
    pub ref_id:i32,
    pub ref_type:i32,//0帖子 1回复
    pub uid:String,
    pub created: i64,
}
rbatis::crud!(LikeBean {},"likes");