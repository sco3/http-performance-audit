use ntex::web;

#[web::get("/")]
async fn index() -> impl web::Responder {
    "Hello, World!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    println!("Ntex server running on http://127.0.0.1:8080");

    web::HttpServer::new(|| async move {
        // Note the 'async move'
        web::App::new().service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
