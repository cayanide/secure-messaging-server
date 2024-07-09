use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::Pool;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation};

// Define your database connection pool
fn establish_connection() -> Pool {
    let url = "mysql://username:password@localhost:3306/database_name";
    mysql::Pool::new(url).expect("Failed to create pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            // Add more routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
