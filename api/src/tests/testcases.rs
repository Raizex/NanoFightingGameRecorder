mod testcases{

    use crate::models::Host;
    use crate::models::Client;
    use crate::models::Response;
    use crate::models::ResponseWithTime;
    use crate::client_handlers;
    use std::sync::Mutex;
    use std::sync::Arc;
    use actix_web::web::Bytes;
    use actix_web::{test, web, App, http::header};

    fn config(cfg: &mut web::ServiceConfig) {
        cfg
        .route("/", web::get().to(client_handlers::index))
        //Configure routes
        .service(
            web::scope("/nano")
                .service(web::resource("/pair").route(web::get().to(client_handlers::pair)))
                //start, stop, and unpair protected routes (client)
                .service(web::resource("/start").route(web::post().to(client_handlers::start)))
                .service(web::resource("/stop").route(web::post().to(client_handlers::stop)))
                .service(web::resource("/unpair").route(web::post().to(client_handlers::unpair)))
        );
    }

    //Test Case 1
    #[actix_rt::test]
    async fn testcase1(){
        // Create new Host object named state for testing
        let state = Arc::new(Mutex::new(Host{is_paired: false, pair_key: "".to_string(), is_recording: false}));

        //Create instance of api web server
        let mut app = test::init_service(App::new().data(state.clone()).configure(config)).await;

        // First step: Pair to api web server
        let req = test::TestRequest::with_uri("/nano/pair").to_request(); // Setup a request with the specified route
        let resp = test::call_service(&mut app, req).await;               // Send request
        assert!(resp.status().is_success());                              // Check if response is a 200 ranged success code              
        

        //Second step: Start recording
        let key_result: Response = test::read_body_json(resp).await; // Grab json response
        let payload = key_result.msg;                                // Strip the msg away from the Response struct
        let client = Client{key: payload};                           // Init and Declare new Client struct

        println!("{}", client.key);
        let resp = test::TestRequest::post()
            .uri("/nano/start")
            .header(header::CONTENT_TYPE, "application/json")
            .set_json(&client)
            .send_request(&mut app)
            .await;
        println!("{}", resp.status().to_string());
        //assert!(resp.status().is_success());


    }
}