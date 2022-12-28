use std::borrow::BorrowMut;

use actix_web::{web, HttpRequest, Responder};
use rbatis::{rbdc::datetime::FastDateTime, sql::PageRequest, Rbatis};
use rust_i18n::t;

/**
 * 🔑Create New Comment
 * @request json
 * @param data: String Comment content
 * @return data: null
 * @code 200 success
 * @code 401 need auth
 * @code 403 permisson denied
 * @code 409 conflict: content exist
 * @code 500 internal error
 */
pub async fn create(
    rb: web::Data<Rbatis>,
    ugc_input: web::Json<UGCInput>,
    user: UserData,
) -> web::Json<Resp<String>> {
    let time = FastDateTime::now().unix_timestamp();
    if let Some(content) = ugc_input.content.clone() {
        let ugc = UGCBean {
            id: None,
            title: ugc_input.title.clone(),
            content: content.clone(),
            uid: user.id,
            likes: 0,
            comments: 0,
            created: time,
            updated: time,
            status: 0,
        };
        let mut rb = rb.get_ref().to_owned();
        let rb = rb.borrow_mut();
        let result = UGCBean::select_by_column(rb, "content", content).await;
        match result {
            Ok(v) => {
                if v.is_empty() {
                    let _result = UGCBean::insert(rb, &ugc).await.unwrap();
                    return success(t!("created"), None);
                } else {
                    return failed(409, t!("exist"));
                }
            }
            Err(err) => return failed(500, t!("error.internal", error = &err.to_string())),
        }
    } else {
        failed(500, t!("error.internal", error = "Unknown"))
    }
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
pub async fn delete(
    req: HttpRequest,
    rb: web::Data<Rbatis>,
    user: UserData,
) -> web::Json<Resp<String>> {
    let id_option = req.match_info().get("id");
    if let Some(id) = id_option {
        let mut rb = rb.get_ref().to_owned();
        let rb = rb.borrow_mut();
        let result = UGCBean::select_by_column(rb, "id", id).await;
        match result {
            Ok(v) => {
                if !v.is_empty() {
                    let ugc = v[0].clone();
                    match user.verify(&(ugc.uid)) {
                        Ok(_) => {
                            let _ = UGCBean::delete_by_column(rb, "id", id).await;
                            let _ = UGCHistoryBean::delete_by_column(rb, "ref_id", id).await;
                            return success(t!("success"), None);
                        }
                        Err(s) => {
                            return s;
                        }
                    }
                } else {
                    return failed(404, t!("error.not_found"));
                }
            }
            Err(err) => return failed(500, t!("error.internal", error = &err.to_string())),
        }
    }
    return failed(405, t!("error.required", name = "Id"));
}


/**
 * 🔑Get All Comments of Specific User Content
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

pub async fn gets(pager: web::Json<Pager>, rb: web::Data<Rbatis>, user: UserData) -> impl Responder {
    let page = pager.page.unwrap_or(1);
    let limit = pager.limit.unwrap_or(20);
    let mut rb = rb.get_ref().to_owned();
    let rb = rb.borrow_mut();
    let req = PageRequest::new(page, limit);
    let data = UGCBean::select_page(rb, &req, &user.id).await;
    match data {
        Ok(v) => {
            if v.page_size > 0 {
                return success(t!("success"), Some(v.records));
            } else {
                return failed(404, t!("error.not_found"));
            }
        }
        Err(err) => return failed(500, t!("error.internal", error = &err.to_string())),
    }
}

use crate::{
    bean::{
        ugc::{UGCBean, UGCInput},
        ugc_history::UGCHistoryBean,
    },
    decoder::{user_data::UserData, pager::Pager},
    response::response::{failed, success, Resp},
};
