use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

mod controller;
mod service;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
            .with_db("gjg")
            .build(),
    );

    let pool = Pool::builder().max_size(20).build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/user", web::post().to_async(controller::user::create))
            .route("/user/{user_id}", web::get().to_async(controller::user::get))
            //.route("/score/submit", web::post().to_async(controller::submit_score))
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .run()
    .unwrap();
}
