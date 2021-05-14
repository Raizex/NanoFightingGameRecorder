use crate::utils;
use crate::models::Status;
use crate::models::Host;
use crate::models::Client;
use crate::models::Response;
use actix_web::{web, Responder, HttpResponse, HttpRequest};
use stopwatch::{Stopwatch};
use std::sync::Mutex;
use std::sync::Arc;
use recorder::recorder::Recorder;

// Client Handlers 
// Parameters: 
// state - This is the host object that holds the overall status of the host device (pair_key, is_recording, is_paired) 
// sw - This is the stopwatch object that records the video time.
// info - This is the json body that the POST requests have included


// Test get request handler which gives status of web server
pub async fn index(_req: HttpRequest) -> impl Responder{
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
        HttpResponse::Unauthorized().json(Response{msg: "Error".to_string()})
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
        HttpResponse::Unauthorized().json(Response{msg: "Error".to_string()})
    }
}


//// POST /nano/start ContentType: applications/json {"key":"{pairkey}"}
pub async fn start(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>, recorder: web::Data<Arc<Mutex<Recorder>>>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();
    let mut recorder = recorder.lock().unwrap();
    println!("Start button pressed");

    if state.pair_key == info.key && state.is_recording == false && state.is_paired == true{
        recorder.record();
        state.is_recording = true;
        sw.reset();
        sw.start();
        HttpResponse::Ok().json(Response{msg: "Success".to_string()})
    }else if state.pair_key == info.key && state.is_recording == true && state.is_paired == true{
        HttpResponse::AlreadyReported().json(Response{msg: "Recording Already Started".to_string()})
    }else{
        HttpResponse::Unauthorized().json(Response{msg: "Error".to_string()})
    }
}

// POST /nano/stop ContentType: applications/json {"key":"{pairkey}"}
pub async fn stop(state: web::Data<Arc<Mutex<Host>>>, sw: web::Data<Arc<Mutex<Stopwatch>>>, info: web::Json<Client>, recorder: web::Data<Arc<Mutex<Recorder>>>) -> impl Responder{
    let mut state = state.lock().unwrap();
    let mut sw = sw.lock().unwrap();
    let mut recorder = recorder.lock().unwrap();
    println!("Stop button pressed");

    if state.pair_key == info.key && state.is_recording == true && state.is_paired == true{
        recorder.stop();
        state.is_recording = false;
        sw.stop();
        HttpResponse::Ok().json(Response{msg: "Success".to_string()})
    }else if state.pair_key == info.key && state.is_recording == false && state.is_paired == true{
        HttpResponse::AlreadyReported().json(Response{msg: "Recording Already Stopped".to_string()})
    }else{
        HttpResponse::Unauthorized().json(Response{msg: "Error".to_string()})
    }
}
