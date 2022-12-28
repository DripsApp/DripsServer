use actix_web::{web, Error};
use rbatis::rbdc::datetime::FastDateTime;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use sled::{Db, IVec};

use std::future::{ready, Ready};

// Cache
lazy_static! {
    static ref CACHE: Db = {
        let tree = sled::open(config::get_kv_path()).expect("open");
        tree
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub expired: i64,
}

impl UserData {
    pub fn verify<T>(&self, target: &str) -> Result<(), web::Json<Resp<T>>>
    where
        T: Serialize,
    {
        let time = FastDateTime::now().unix_timestamp();
        if time < self.expired {
            if target == self.id.as_str() {
                return Ok(());
            } else {
                return Err(failed(403, t!("error.permission_denied")));
            }
        }
        Err(failed(401, t!("error.not_authorization")))
    }
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

use crate::{
    config,
    response::response::{failed, Resp},
};
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
                        Err(error::ErrorBadRequest(t!("error.invalid_authorization")))
                    }
                }
            } else {
                Err(error::ErrorUnauthorized(t!("error.authorization_not_found")))
            }
        })
    }
}

fn validate_token(token: &str) -> Result<UserData, String> {
    let now = chrono::prelude::Local::now();

    let v = CACHE.get(token);
    if let Ok(Some(u)) = v {
        let user: UserData = serde_json::from_slice(u.to_vec().as_slice()).unwrap();
        if user.expired > now.timestamp() {
            return Ok(user.clone());
        } else {
            return request_token(token);
        }
    }
    return request_token(token);
}

// todo
fn request_token(token: &str) -> Result<UserData, String> {
    let user = UserData {
        id: format!("{}", token),
        expired: 2792909380,
    };
    let str = serde_json::to_string(&user).unwrap();
    let result = CACHE.insert(token.to_string(), IVec::from(str.as_bytes()));
    match result {
        Ok(_) => Ok(user),
        Err(v) => Err(v.to_string()),
    }
}
