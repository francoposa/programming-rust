use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

mod gcd;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        App::new()
            .route("/", web::get().to(get_calc))
            .route("/gcd", web::post().to(post_gcd))
    };
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

fn get_calc() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>
            "#,
    )
}

#[derive(Deserialize)]
struct GCDParams {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GCDParams>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Come on now... you know you can't use zero");
    }
    let body = format!(
        "The Greatest Common Divisor of {} and {} \
        is <b>{}</b>\n",
        form.n,
        form.m,
        gcd::gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(body)
}
