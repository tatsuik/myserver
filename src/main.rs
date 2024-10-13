use actix_web::{get, post, test, web, body, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}

#[cfg(test)]
#[actix_web::test]
async fn test_hello() {
    let app = test::init_service(App::new().service(hello)).await;

    let req = test::TestRequest::get().to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = resp.into_body();
    let bytes = body::to_bytes(body).await;
    assert_eq!(bytes.unwrap(), web::Bytes::from_static(b"Hello, world!"))
}

