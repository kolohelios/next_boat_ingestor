use actix_web::{get, web, App, HttpServer, Responder};

#[get("/index.html")]
async fn root_index() -> impl Responder {
    format!("Hello, world!")
}

#[get("/{id}/{name}/index.html")]
async fn index(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(rootIndex))
        .bind(("127.0.0.1", 5001))?
        .run()
        .await
}
