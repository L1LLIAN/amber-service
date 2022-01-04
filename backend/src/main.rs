mod services;

use std::env;

use actix_web::{
    get,
    http::header::ContentType,
    web::{self, Bytes},
    App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use services::PictureService;

#[get("/")]
async fn index(data: web::Data<PictureService>) -> impl Responder {
    let result = &data.get_picture();

    match result {
        Some(vec) => {
            let bytes = Bytes::copy_from_slice(vec);
            HttpResponse::Ok().set(ContentType::png()).body(bytes)
        }

        None => HttpResponse::BadRequest().body("404 Not Found"),
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
