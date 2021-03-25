mod models;
mod config;
mod utils;

use crate::models::Status;
use crate::models::Host;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use stopwatch::{Stopwatch};
use std::sync::Mutex;
use std::sync::Arc;
use dotenv::dotenv;


// Test get request handler which gives status of web server
async fn index() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{status: "UP".to_string()})
}

//Pair to api 
async fn pair(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
    let mut state = state.lock().unwrap();
    if state.is_paired == false{
        state.is_paired = true;
        state.pair_key = utils::generate();
        HttpResponse::Ok().body(format!("{}", state.pair_key))
    }else {
        HttpResponse::Ok().body("Access Denied")
    }
}

//Return client pair_key
async fn getkey(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
    let state = state.lock().unwrap();
    HttpResponse::Ok().body(format!("{}", state.pair_key))
}

//Unpair client device with api
async fn unpair(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, req: HttpRequest) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();
    let input_key: String = req.match_info().get("key").unwrap().parse().unwrap();

    if state.pair_key == input_key && state.is_paired == true && state.is_paired == true{
        state.is_recording = false;
        state.pair_key = "".to_string();
        state.is_paired = false;
        sw.reset();
        HttpResponse::Ok().body("Disconnected from Jetson Nano")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

async fn testidentity(id: Identity) -> impl Responder{
    HttpResponse::Ok().body(format!("Hello {}", id.identity().unwrap_or_else(|| "Anonymous".to_owned())))
}

async fn checkin(id: Identity) -> impl Responder{
    id.remember("PairedClient".to_owned());
    HttpResponse::Ok().body("Remembering User...")
}

async fn checkout(id: Identity) -> impl Responder{
    id.forget();
    HttpResponse::Ok().body("Goodbye")
}

//Start recording request handler
async fn start(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, req: HttpRequest) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();
    let input_key: String = req.match_info().get("key").unwrap().parse().unwrap();

    if state.pair_key == input_key && state.is_recording == false && state.is_paired == true{
        state.is_recording = true;
        sw.reset();
        sw.start();
        HttpResponse::Ok().body("Access Granted. Recording Has Started")
    }else if state.pair_key == input_key && state.is_recording == true && state.is_paired == true{
        HttpResponse::Ok().body("Access Granted. Recording is already in progress!")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

//Stop recording request handler
async fn stop(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, req: HttpRequest) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();
    let input_key: String = req.match_info().get("key").unwrap().parse().unwrap();

    if state.pair_key == input_key && state.is_recording == false && state.is_paired == true{
        HttpResponse::Ok().body(format!("Access Granted. Recording has not yet started. Time Elapsed: {}", sw.elapsed_ms()))
    }else if state.pair_key == input_key && state.is_recording == true && state.is_paired == true{
        state.is_recording = false;
        sw.stop();
        HttpResponse::Ok().body(format!("Access Granted. Recording has stopped. Time Elapsed: {}", sw.elapsed_ms()))
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    //Make sure stopwatch is set to zero
    let sw = Arc::new(Mutex::new(Stopwatch::start_new()));
    sw.lock().unwrap().reset();
    
    //Create a current_state object that is instantiated from the Host struct
    //This current_state object will hold an 'is_paired' boolean and a pair_key
    let state = Arc::new(Mutex::new(Host{is_paired: false, pair_key: "".to_string(), is_recording: false}));

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);
    println!("Admin key: {}", state.lock().unwrap().pair_key);

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)
            ))
            //Default Routes
            .route("/", web::get().to(index))
            .route("/checkin", web::get().to(checkin))
            .route("/testid", web::get().to(testidentity))
            .route("/checkout", web::get().to(checkout))
            //Configure routes
            .service(
                web::scope("/nano")
                    //Guest endpoint (Pair Command)
                    .data(state.clone())
                    .data(sw.clone())
                    .service(web::resource("/pair").route(web::get().to(pair)))
                    //Admin Scope
                    .service(
                        web::scope("/admin")
                            .service(web::resource(format!("/getkey/{}", state.lock().unwrap().pair_key)).route(web::get().to(getkey)))
                    )
                    //start, stop, and unpair protected routes (client)
                    .service(web::resource("/start/{key}").route(web::get().to(start)))
                    .service(web::resource("/stop/{key}").route(web::get().to(stop)))
                    .service(web::resource("/unpair/{key}").route(web::get().to(unpair)))
                
            )
    
    })
    .bind(format!("{}:{}",config.server.host, config.server.port))?
    .run()
    .await
}

