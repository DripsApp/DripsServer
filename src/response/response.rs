use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct Resp<T>
where
    T: Serialize,
{
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<T>,
}

pub fn success<T>(msg: String, data: Option<T>) -> web::Json<Resp<T>>
where
    T: Serialize,
{
    web::Json(Resp {
        code: 200,
        msg: Some(msg),
        data,
    })
}
pub fn success_with_code<T>(code:i32,msg: String, data: Option<T>) -> web::Json<Resp<T>>
where
    T: Serialize,
{
    web::Json(Resp {
        code,
        msg: Some(msg),
        data,
    })
}


pub fn failed<T>(code: i32, msg: String) -> web::Json<Resp<T>>
where
    T: Serialize,
{
    web::Json(Resp {
        code,
        msg: Some(msg),
        data: None,
    })
}
