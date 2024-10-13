use actix_web::{body, get, http::{self, header::{self, TryIntoHeaderPair}}, post, test, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct SensorData {
    date: u32,
    time: u32,
    temperature: f32,
    humidity: f32,
}

#[get("/data")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok().json(SensorData {
        date: 20241012,
        time: 210000,
        temperature: 21.3,
        humidity: 42.4
    })
}

#[post("/data")]
async fn post_data(data: web::Json<SensorData>) -> impl Responder {
    println!{"{:?}", data};
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(post_data)
            .service(get_data)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}

#[cfg(test)]
#[actix_web::test]
async fn test_get_data() {
    let app = test::init_service(App::new().service(get_data)).await;

    let req = test::TestRequest::get().uri("/data").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let json : SensorData = test::read_body_json(resp).await;
    assert_eq!(json.date, 20241012);
    assert_eq!(json.time, 210000);
    assert_eq!(json.temperature, 21.3);
    assert_eq!(json.humidity, 42.4);
}

#[actix_web::test]
async fn test_post_data() {
    let mut app = test::init_service(App::new().service(post_data)).await;
    let data = SensorData {
        date: 20241013, 
        time:225040, 
        temperature:19.2, 
        humidity:45.0
    };
    let res = test::TestRequest::post()
        .uri("/data")
        .append_header(header::ContentType::json())
        .set_json(data)
        .send_request(&mut app).await;

    assert!(res.status().is_success());
}
