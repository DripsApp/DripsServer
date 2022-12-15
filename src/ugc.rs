use actix_web::{HttpRequest, Responder};

/**
 * 🔑Create New User Content
 * @request json
 * @param data: UGC
 * @return data: null
 * @code 200 success
 * @code 401 need auth
 * @code 403 permisson denied
 * @code 409 conflict: content exist
 * @code 500 internal error
 */
pub async fn create(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/**
 * 🔑Update Specific User Content
 * @request json
 * @param data: UGC
 * @return data: null
 * @code 200 success
 * @code 401 need auth
 * @code 403 permisson denied
 * @code 404 not found
 * @code 500 internal error
 */
pub async fn update(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/**
 * 🔑Delete Specific User Content
 * @request json
 * @param id: string specific user content id
 * @return null
 * @code 200 success
 * @code 401 need auth
 * @code 403 permisson denied
 * @code 404 not found
 * @code 500 internal error
 */
pub async fn delete(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/**
 * 🔑Get Specific User Content
 * @request json
 * @param id: string specific user content id
 * @return data: UGC
 * @code 200 success
 * @code 401 need auth
 * @code 403 permisson denied
 * @code 404 not found
 * @code 500 internal error
 */
pub async fn get(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {:?}!", req)
}

/**
 * 🔑Get Specific User Content
 * @request json
 * @param page: int
 * @param limit: int
 * @return data: [UGC]
 * @code 200 success
 * @code 401 need auth
 * @code 403 permisson denied
 * @code 404 not found
 * @code 500 internal error
 */
pub async fn gets(req: HttpRequest, user: Option<UserData>) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    // serde_json::to_string(&UGC {
    //     id: 100,
    //     title: "Title".to_string(),
    //     content: "User generates content".to_string(),
    //     uid: "a3d13f".to_string(),
    //     likes: 2,
    //     comments: 1,
    //     created: 1009771214,
    //     updated: 1009771214,
    // })
    // .unwrap()
    if let Some(u) = user {
        serde_json::to_string(&u).unwrap()
    }else{
        String::from("Error")
    }
}

use serde::{Deserialize, Serialize};

use crate::auth::UserData;

#[derive(Serialize, Deserialize, Debug)]
struct UGC {
    id: i32,
    title: String,
    content: String,
    uid: String,
    likes: i32,
    comments: i32,
    created: i32,
    updated: i32,
}
