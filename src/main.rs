use actix_web::{App, get, HttpServer, Responder, web, HttpResponse};

#[get("/")]
fn index() -> impl Responder {
    format!("Hello world")
}

fn main() -> std::io::Result<()> {
//    let sys = actix_rt::System::new("example");  // <- create Actix runtime

    HttpServer::new(
        || App::new()
            .service(index)

    )
        .bind("0.0.0.0:59090")?
        .run()

//    actix_rt::System::current().stop();
//    sys.run()
}