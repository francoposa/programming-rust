use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || App::new().route("/", web::get().to(get_index));
    let server = HttpServer::new(app)
        .bind("127.0.0.1:8080")
        .expect("failed to bind on 127.0.0.1:8080");

    println!("running http server on 127.0.0.1:8080...");
    server.run().await
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Hello World")
}
