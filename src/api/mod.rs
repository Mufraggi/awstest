mod health;

use crate::api::health::health;
use serde::Serialize;
use serde::Deserialize;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};


pub async fn serve(
    url: &str,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
           // .wrap(RequestTracing::new())
            .service(web::scope("/health").route("", web::get().to(health)))
            .service(web::scope("/").route("", web::get().to(coucou)))
    })
    .bind((url, 8080))?
    .run()
    .await
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
struct Response {
    status: String,
}

pub async fn coucou() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&Response {
            status: "coucou".parse().unwrap()
        }).unwrap())
}