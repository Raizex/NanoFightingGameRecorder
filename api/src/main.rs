
mod config;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
//use std::io;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn index() -> impl Responder{
    HttpResponse::Ok().body("Hello sunshine!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(index))
    
    })
    .bind(format!("{}:{}",config.server.host, config.server.port))?
    .run()
    .await
}