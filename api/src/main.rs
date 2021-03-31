mod models;
mod config;
mod utils;
mod client_handlers;

use crate::models::Host;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use stopwatch::{Stopwatch};
use std::sync::Mutex;
use std::sync::Arc;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    //Declare a configurations variable that derives from the .env file
    //So that we can retrieve our server and port information
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    //Make sure stopwatch is set to zero
    let sw = Arc::new(Mutex::new(Stopwatch::start_new()));
    sw.lock().unwrap().reset();

    //Load ssl keys by creating a self-signed temporary cert for testing
    //`openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    
    //Create a current_state object that is instantiated from the Host struct
    //This current_state object will hold an 'is_paired' boolean and a pair_key
    let state = Arc::new(Mutex::new(Host{is_paired: false, pair_key: "".to_string(), is_recording: false}));

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);
    println!("Admin key: {}", state.lock().unwrap().pair_key);

    HttpServer::new(move || {
        App::new()
            //Default Routes
            .route("/", web::get().to(client_handlers::index))
            //Configure routes
            .service(
                web::scope("/nano")
                    //Guest endpoint (Pair Command)
                    .data(state.clone())
                    .data(sw.clone())
                    .service(web::resource("/pair").route(web::get().to(client_handlers::pair)))
                    //Admin Scope
                    .service(
                        web::scope("/admin")
                            .service(web::resource(format!("/getkey/{}", state.lock().unwrap().pair_key)).route(web::get().to(client_handlers::getkey)))
                    )
                    //start, stop, and unpair protected routes (client)
                    .service(web::resource("/start").route(web::post().to(client_handlers::start)))
                    .service(web::resource("/stop").route(web::post().to(client_handlers::stop)))
                    .service(web::resource("/unpair").route(web::post().to(client_handlers::unpair)))
                    .service(web::resource("/test").route(web::post().to(client_handlers::test)))
                
            )
    
    })
    .bind_openssl(format!("{}:{}",config.server.host, config.server.port), builder)?
    .run()
    .await
}

