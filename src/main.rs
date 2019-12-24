#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    Result,
};
use telegram_typing_bot::typing::UpdateMessage;

#[post("/telegram")]
async fn hello(update: web::Json<telegram_typing_bot::typing::Update>) -> impl Responder {
    match &update.message {
        UpdateMessage::InlineQuery(msg) => {}
        _ => {}
    };
    "OK"
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "pypirobot=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .start()
    .await
}
