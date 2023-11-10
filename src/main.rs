use actix_web::{ get, App, HttpResponse, HttpServer, Responder };

#[get("/ping")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;

    println!("Starting server at {}:{}", host, port);

    HttpServer::new(|| App::new().service(hello))
        .bind((host, port))?
        .run().await
}
