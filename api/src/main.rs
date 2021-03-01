mod models;
mod config;

use crate::models::Status;
use crate::models::Host;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::Mutex;
use std::sync::Arc;
use rand::Rng;
use dotenv::dotenv;

async fn index() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{status: "UP".to_string()})
}

async fn pair(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
    let mut state = state.lock().unwrap();
    if state.is_paired == false{
        state.is_paired = true;
        state.pair_key = generate();
        HttpResponse::Ok().body(format!("pair_key: {}", state.pair_key))
    }else {
        HttpResponse::Ok().body("Access Denied!")
    }
}

async fn unpair() -> impl Responder{
    HttpResponse::Ok().body("Access Denied!")
}

async fn start() -> impl Responder{
    HttpResponse::Ok().body("Access Denied!")
}

async fn stop() -> impl Responder{
    HttpResponse::Ok().body("Access Denied!")
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
    let state = Arc::new(Mutex::new(Host{is_paired: false, pair_key: key}));

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            //Default Routes
            .route("/", web::get().to(index))
            //Configure routes
            .service(
                web::scope("/nano")
                    //Guest endpoint (Pair Command)
                    .data(state.clone())
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
