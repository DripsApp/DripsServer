use actix_web::Error;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    future::{ready, Ready},
    sync::Mutex,
};

// Redis
lazy_static! {
    static ref CACHE: Mutex<HashMap<String, UserData>> = {
        let map: HashMap<String, UserData> = HashMap::new();
        Mutex::new(map)
    };
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    id: String,
    expired: i64,
}

/**
 * Authorization
 * If KV Database Has it, use it.
 * If not or expires, request from user server and storage/update it in KV Database.
 * [middleware]
 * @param token: string
 * @return { uid:string,expired:timestamp(sec) }
 * @code 401 need auth
 */
use actix_web::{dev::Payload, error, FromRequest, HttpRequest};
impl FromRequest for UserData {
    type Error = Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready({
            let auth = req.headers().get("Authorization");
            if let Some(val) = auth {
                let token = val
                    .to_str()
                    .unwrap()
                    .split("Bearer ")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap();
                let result = validate_token(token);
                match result {
                    Ok(data) => Ok(data),
                    Err(e) => {
                        eprintln!("{}", e);
                        Err(error::ErrorBadRequest("Invalid Authorization"))
                    }
                }
            } else {
                Err(error::ErrorUnauthorized("Authorization Not Found"))
            }
        })
    }
}

fn validate_token(token: &str) -> Result<UserData, String> {
    let now = chrono::prelude::Local::now();
    let v = CACHE.lock().unwrap().clone();
    let v = v.get(token);
    if let Some(user) = v {
        if user.expired > now.timestamp() {
            return Ok((UserData {
                id: format!("Cache{}", user.id),
                expired: user.expired,
            }).clone());
        } else {
            return request_token(token);
        }
    }
    return request_token(token);
}

// todo
fn request_token(token: &str) -> Result<UserData, String> {
    let user = UserData {
        id: format!("({})", token),
        expired: 2792909380,
    };
    CACHE
        .lock()
        .unwrap()
        .insert(token.to_string(), user.clone());
    Ok(user)
}
