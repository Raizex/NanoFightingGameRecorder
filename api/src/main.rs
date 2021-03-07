mod models;
mod config;

use crate::models::Status;
use crate::models::Host;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use std::sync::Mutex;
use std::sync::Arc;
use rand::Rng;
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
        state.pair_key = generate();
        HttpResponse::Ok().body(format!("pair_key: {}", state.pair_key))
    }else {
        HttpResponse::Ok().body("Access Denied!")
    }
}

//Return client pair_key
async fn getkey(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
    let state = state.lock().unwrap();
    HttpResponse::Ok().body(format!("{}", state.pair_key))
}

//Unpair client device with api
async fn unpair(state: web::Data<Arc<Mutex<Host>>>, req: HttpRequest) -> impl Responder{
    let mut state = state.lock().unwrap();
    let input_key: String = req.match_info().get("key").unwrap().parse().unwrap();

    if state.pair_key == input_key && state.is_paired == true{
        state.is_recording = false;
        state.pair_key = generate();
        state.is_paired = false;
        HttpResponse::Ok().body("Disconnected from Jetson Nano")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

//Start recording request handler
async fn start(state: web::Data<Arc<Mutex<Host>>>, req: HttpRequest) -> impl Responder{
    let mut state = state.lock().unwrap();
    let input_key: String = req.match_info().get("key").unwrap().parse().unwrap();

    if state.pair_key == input_key && state.is_recording == false{
        state.is_recording = true;
        HttpResponse::Ok().body("Access Granted. Recording Has Started")
    }else if state.pair_key == input_key && state.is_recording == true{
        HttpResponse::Ok().body("Access Granted. Recording is already in progress!")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

//Stop recording request handler
async fn stop(state: web::Data<Arc<Mutex<Host>>>, req: HttpRequest) -> impl Responder{
    let mut state = state.lock().unwrap();
    let input_key: String = req.match_info().get("key").unwrap().parse().unwrap();

    if state.pair_key == input_key && state.is_recording == false{
        HttpResponse::Ok().body("Access Granted. Recording has not yet started.")
    }else if state.pair_key == input_key && state.is_recording == true{
        state.is_recording = false;
        HttpResponse::Ok().body("Access Granted. Recording has stopped")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
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
    let state = Arc::new(Mutex::new(Host{is_paired: false, pair_key: key, is_recording: false}));

    println!("Starting Server at http://{}:{}/", config.server.host, config.server.port);
    println!("Admin key: {}", state.lock().unwrap().pair_key);

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


//Function generates a unique key
fn generate() -> String{

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
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
