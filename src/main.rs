use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use drips_server::bean::init_db;
use drips_server::ugc;
use env_logger::Env;
use rust_i18n::t;
use drips_server::translate;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rust_i18n::set_locale("zh-CN");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let rb = init_db().await;
    println!("{}",t!("server_start"));
    HttpServer::new(move || {
        App::new().service(
            web::scope("/v1")
                .app_data(web::Data::new(rb.to_owned()))
                .wrap(Logger::new(
                    "[%a] %r => %s[%b B/%D ms]\n[Error]%{X-MSG}o\n%{User-Agent}i",
                ))
                .service(
                    // V1 RESTful API Interface
                    web::scope("/ugc")
                        // UGC API Interface
                        .service(
                            web::resource("")
                                .route(web::post().to(ugc::create))
                                .route(web::get().to(ugc::gets)),
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::get().to(ugc::get))
                                .route(web::put().to(ugc::update))
                                .route(web::delete().to(ugc::delete)),
                        )
                )
                .service(
                    // V1 RESTful API Interface
                    web::scope("/comment")
                        // UGC API Interface
                        .service(
                            web::resource("")
                                .route(web::post().to(ugc::create))
                                .route(web::get().to(ugc::gets)),
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::delete().to(ugc::delete)),
                        )
                )
                .service(
                    // V1 RESTful API Interface
                    web::scope("/like")
                        // UGC API Interface
                        .service(
                            web::resource("")
                                .route(web::post().to(ugc::create))
                                .route(web::get().to(ugc::get)),
                        )
                        .service(
                            web::resource("/{id}")
                                .route(web::delete().to(ugc::delete)),
                        )
                ),
        )
    })
    // .bind(("[::]", 34802))?
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}

// UGC -> Create Read List Update Delete
// UGC -> POST    GET  GET   PUT  DELETE
// UGC -> Like Comment
