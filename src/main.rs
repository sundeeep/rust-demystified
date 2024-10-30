// Import statements:
use actix_web::{get, App, HttpServer, Responder};
use std::io::Result;

mod models;
mod utils;
mod ownership;

use utils::greet::welcome;
use ownership::strings::init_ownership;

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello, drugbord.ai")
    
}

#[actix_web::main]
async fn main() -> Result<()> {
    
    const PORT: u16 = 8000;
    const LOCALHOST: &str = "127.0.0.1";
    println!();

    welcome();



    init_ownership();
    // println!("HTTP Server is running on port: {}", PORT);
    
    HttpServer::new(|| App::new().service(greet))
        .bind((LOCALHOST, PORT))? // returns "Result" => ?
        .workers(2)
        .run()
        .await
}
