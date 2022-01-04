mod services;

use std::env;

use actix_web::{
    get,
    http::header::ContentType,
    post,
    web::{self, Buf, Bytes},
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use services::PictureService;

struct Context {
    auth_token: String,
    picture_service: PictureService,
}

#[get("/")]
async fn get(data: web::Data<Context>) -> impl Responder {
    let result = &data.picture_service.get_picture();

    match result {
        Some(vec) => {
            let bytes = Bytes::copy_from_slice(vec);
            HttpResponse::Ok().set(ContentType::png()).body(bytes)
        }

        None => HttpResponse::BadRequest().body("404 Not Found"),
    }
}

#[post("/")]
async fn post(request: HttpRequest, body: Bytes, data: web::Data<Context>) -> impl Responder {
    let headers = request.headers();
    if !headers.contains_key("Authorization") {
        return HttpResponse::Forbidden();
    }

    let authorization_token = headers
        .get("Authorization")
        .unwrap()
        .to_str()
        .expect("Couldn't convert header value to ascii.");
    if data.auth_token != authorization_token {
        return HttpResponse::Forbidden();
    }

    let content_type = request.content_type();

    // Should probably just verify the magic bytes for the file but, since this is a trusted source it's okay
    if !content_type.starts_with("image/") {
        return HttpResponse::BadRequest();
    }

    match &data.picture_service.save_picture(body.bytes()) {
        Ok(_) => HttpResponse::Ok(),

        Err(why) => {
            println!("ERR: {}", why);
            HttpResponse::InternalServerError()
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
        let auth_token =
            env::var("AUTH_TOKEN").expect("Couldn't get AUTH_TOKEN environment variable!");
        let picture_service = PictureService::new(picture_path.clone());
        let context = Context {
            auth_token,
            picture_service,
        };

        App::new().data(context).service(get).service(post)
    })
    .bind(bind_addr)?
    .run()
    .await
}
