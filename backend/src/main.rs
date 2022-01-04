mod services;

use std::env;

use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use services::PictureService;

#[get("/")]
async fn index() -> impl Responder {
    "Meow!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let picture_path =
        env::var("PICTURE_PATH").expect("Couldnt get PICTURE_PATH environment variable!");

    let bind_addr = env::var("BIND_ADDR").expect("Couldn't get BIND_ADDR environment variable!");
    HttpServer::new(move || {
        let picture_service = PictureService::new(picture_path.clone());

        App::new().data(picture_service).service(index)
    })
    .bind(bind_addr)?
    .run()
    .await
}
