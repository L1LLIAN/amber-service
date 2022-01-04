use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;

#[get("/")]
async fn index() -> impl Responder {
    "Meow!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
