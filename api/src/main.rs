mod models;
mod config;

use crate::models::Status;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
//use std::io;
use dotenv::dotenv;

#[get("/hello")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn index() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{status: "UP".to_string()})
}

async fn pair() -> impl Responder{
    HttpResponse::Ok().body("Device paired")
}

async fn admin() -> impl Responder{
    HttpResponse::Ok().body("Admin access")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(|| {
        App::new()
            //Default Routes
            .service(hello)
            .service(echo)
            .route("/", web::get().to(index))
            //Configure routes
            .service(
                web::scope("/api")
                    //Guest endpoint (Pair Command)
                    .service(web::resource("/pair").route(web::get().to(pair)))
                    //Admin Scope
                    .service(
                        web::scope("/admin")
                            .service(web::resource("/").route(web::get().to(admin)))
                    )
                    //Request access to become the authenticated user for the Jetson Nano
                    .service(
                        web::scope("/auth_user")
                            .service(web::resource("/pair").route(web::get().to(pair)))
                    )
                
            )
    
    })
    .bind(format!("{}:{}",config.server.host, config.server.port))?
    .run()
    .await
}
