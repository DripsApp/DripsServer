use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use drips_server::ugc;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("Drips Content Server Running!");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/v1")
                .wrap(Logger::new(
                    "[%a] %r => %s[%b B/%D ms]\n[Error]%{X-MSG}o\n%{User-Agent}i",
                ))
                .service(
                    // V1 RESTful API Interface
                    web::scope("/ugc")
                        // UGC API Interface
                        .service(web::resource("").route(web::get().to(ugc::gets)))
                        .service(
                            web::resource("/{id}")
                                .route(web::post().to(ugc::create))
                                .route(web::get().to(ugc::get))
                                .route(web::put().to(ugc::update))
                                .route(web::delete().to(ugc::delete)),
                        ),
                ),
        )
    })
    .bind(("[::]", 34802))?
    .bind(("0.0.0.0", 34802))?
    .run()
    .await
}

// UGC -> Create Read List Update Delete
// UGC -> POST    GET  GET   PUT  DELETE
// UGC -> Like Comment
