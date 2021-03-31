use crate::utils;
use crate::models::Status;
use crate::models::Host;
use crate::models::Client;
use actix_web::{web, Responder, HttpResponse};
use stopwatch::{Stopwatch};
use std::sync::Mutex;
use std::sync::Arc;

// Test get request handler which gives status of web server
pub async fn index() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{status: "UP".to_string()})
}

//Pair to api 
pub async fn pair(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
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
pub async fn getkey(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
    let state = state.lock().unwrap();
    HttpResponse::Ok().body(format!("{}", state.pair_key))
}

//Unpair client device with api
pub async fn unpair(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();

    if state.pair_key == info.key && state.is_paired == true && state.is_paired == true{
        state.is_recording = false;
        state.pair_key = "".to_string();
        state.is_paired = false;
        sw.reset();
        HttpResponse::Ok().body("Disconnected from Jetson Nano")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

pub async fn test(state: web::Data<Arc<Mutex<Host>>>, info: web::Json<Client>) -> impl Responder {
    let mut state = state.lock().unwrap();


    if info.key == state.pair_key{
        HttpResponse::Ok().body(format!("{}", info.key))
    }else{
        HttpResponse::Ok().body("fail")
    }
    
}

//Start recording request handler
pub async fn start(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();

    if state.pair_key == info.key && state.is_recording == false && state.is_paired == true{
        state.is_recording = true;
        sw.reset();
        sw.start();
        HttpResponse::Ok().body("Access Granted. Recording Has Started")
    }else if state.pair_key == info.key && state.is_recording == true && state.is_paired == true{
        HttpResponse::Ok().body("Access Granted. Recording is already in progress!")
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}

//Stop recording request handler
pub async fn stop(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();

    if state.pair_key == info.key && state.is_recording == false && state.is_paired == true{
        //Create a time_list vector that recieves the converted time in minutes and seconds
        //The first element in the list are the minutes and the second element are your seconds
        let time_list: Vec<i64> = utils::convert_time(sw.elapsed_ms());
        HttpResponse::Ok().body(format!("Access Granted. Recording has not yet started. Time Elapsed: {}:{}", time_list[0], time_list[1]))
    }else if state.pair_key == info.key && state.is_recording == true && state.is_paired == true{
        state.is_recording = false;
        sw.stop();
        let time_list: Vec<i64> = utils::convert_time(sw.elapsed_ms());
        HttpResponse::Ok().body(format!("Access Granted. Recording has stopped. Time Elapsed: {}:{}", time_list[0], time_list[1]))
    }else{
        HttpResponse::Ok().body("Access Denied")
    }
}