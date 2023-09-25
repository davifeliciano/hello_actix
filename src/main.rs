use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod config;
mod database;
mod errors;
mod handlers;
mod models;

use crate::config::Config;
use crate::handlers::{count_people, create_person, get_people, get_person};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let port = config.port.unwrap_or(5000);
    let addr = "127.0.0.1";

    let server = HttpServer::new(move || {
        let json_config = Config::json_extractor_config();

        App::new()
            .app_data(json_config)
            .app_data(web::Data::new(pool.clone()))
            .service(count_people)
            .service(
                web::scope("/pessoas")
                    .service(get_people)
                    .service(get_person)
                    .service(create_person),
            )
    })
    .bind((addr, port))?
    .run();

    println!("Server running on port {port}");

    server.await
}
