use crate::utils;
use crate::models::Status;
use crate::models::Host;
use crate::models::Client;
use crate::models::Response;
use crate::models::ResponseWithTime;
use actix_web::{web, Responder, HttpResponse};
use stopwatch::{Stopwatch};
use std::sync::Mutex;
use std::sync::Arc;

// Client Handlers 
// Parameters: 
// state - This is the host object that holds the overall status of the host device (pair_key, is_recording, is_paired) 
// sw - This is the stopwatch object that records the video time.
// info - This is the json body that the POST requests have included


// Test get request handler which gives status of web server
pub async fn index() -> impl Responder{
    HttpResponse::Ok()
        .json(Status{status: "UP".to_string()})
}


//GET /nano/pair
pub async fn pair(state: web::Data<Arc<Mutex<Host>>>) -> impl Responder{
    let mut state = state.lock().unwrap();
    if state.is_paired == false{
        state.is_paired = true;
        state.pair_key = utils::generate();
        HttpResponse::Ok().json(Response{msg: state.pair_key.to_string()})
    }else {
        HttpResponse::Ok().json(Response{msg: "Error".to_string()})
    }
}

// POST /nano/unpair ContentType: applications/json {"key":"{pairkey}"}
pub async fn unpair(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();

    if state.pair_key == info.key && state.is_paired == true && state.is_paired == true{
        state.is_recording = false;
        state.pair_key = "".to_string();
        state.is_paired = false;
        sw.reset();
        HttpResponse::Ok().json(Response{msg: "Disconnected".to_string()})
    }else{
        HttpResponse::Ok().json(Response{msg: "Error".to_string()})
    }
}


//// POST /nano/start ContentType: applications/json {"key":"{pairkey}"}
pub async fn start(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();

    if state.pair_key == info.key && state.is_recording == false && state.is_paired == true{
        state.is_recording = true;
        sw.reset();
        sw.start();
        HttpResponse::Ok().json(Response{msg: "Success".to_string()})
    }else if state.pair_key == info.key && state.is_recording == true && state.is_paired == true{
        HttpResponse::Ok().json(Response{msg: "Recording Already Started".to_string()})
    }else{
        HttpResponse::Ok().json(Response{msg: "Error".to_string()})
    }
}

// POST /nano/stop ContentType: applications/json {"key":"{pairkey}"}
pub async fn stop(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();

    if state.pair_key == info.key && state.is_recording == true && state.is_paired == true{
        state.is_recording = false;
        sw.stop();
        //Create a time_list vector that recieves the converted time in minutes and seconds
        //The first element in the list are the minutes and the second element are your seconds
        let time_list: Vec<i64> = utils::convert_time(sw.elapsed_ms());
        HttpResponse::Ok().json(ResponseWithTime{msg: "Success".to_string(), time: format!("{}:{}", time_list[0], time_list[1])})
    }else if state.pair_key == info.key && state.is_recording == false && state.is_paired == true{
        let time_list: Vec<i64> = utils::convert_time(sw.elapsed_ms());
        HttpResponse::Ok().json(ResponseWithTime{msg: "Recording Already Stopped".to_string(), time: format!("{}:{}", time_list[0], time_list[1])})
    }else{
        HttpResponse::Ok().json(Response{msg: "Error".to_string()})
    }
}