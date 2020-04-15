use actix_web::{HttpServer, App, web};

mod hello;
mod greet;

pub fn start() {
    let bind_address = "127.0.0.1:8080";
    HttpServer::new(|| {
        App::new().configure(routes)
        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run();

    println!("You can access the server at {}", &bind_address);
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .route("/", web::get().to(hello::get))
        .route("/greeting/{name}", web::get().to(greet::get));
}

