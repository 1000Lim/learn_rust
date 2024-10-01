use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Lin")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn mannual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)  // Register the echo handler
            .service(say_hello)
            .route("/hey", actix_web::web::get().to(mannual_hello))
    })
    .bind("127.0.0.1:8080")?  // Bind server to localhost on port 8080
    .run()
    .await
}
