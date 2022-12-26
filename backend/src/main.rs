use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_lab::web::spa;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish(),
        )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
