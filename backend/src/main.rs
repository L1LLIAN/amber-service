mod services;

use std::env;

use actix_web::{get, web::{self, Bytes}, App, HttpServer, Responder};
use dotenv::dotenv;
use services::PictureService;

#[get("/")]
async fn index(data: web::Data<PictureService>) -> impl Responder {
    let result = &data.get_picture();
    match result {
        Some(vec) => {
            Bytes::copy_from_slice(vec)
        }

        None => {
            Bytes::new()
        }
    }
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
