mod models;
mod config;

use crate::models::Status;
use crate::models::Host;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
//use std::io;
use rand::Rng;
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

async fn pair(state: Host) -> impl Responder{
    HttpResponse::Ok().body("Device paired")
}

async fn admin() -> impl Responder{
    HttpResponse::Ok().body("Admin access")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    //Instantiate and set pair_key value by calling the generate function
    //Create a current_state object that is instantiated from the Host struct
    //This current_state object will hold an 'is_paired' boolean and a pair_key
    let key = generate();
    let current_state = Host::new(false, key);

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            //Default Routes
            .service(hello)
            .service(echo)
            .route("/", web::get().to(index))
            //Configure routes
            .service(
                web::scope("/nano")
                    //Guest endpoint (Pair Command)
                    .data(current_state.copy())
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


//Function generates a unique key
fn generate() -> String{

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}
