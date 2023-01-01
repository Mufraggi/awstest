use std::env;
use crate::api::serve;

mod api;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url_domain = env::var("URL_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    serve(&url_domain).await
}
