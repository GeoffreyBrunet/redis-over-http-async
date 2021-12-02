mod routes;
mod structs;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use routes::cache::{delete_cache, get_cache, post_cache};
use routes::health_check::health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(100).build(manager);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/health_check", web::get().to(health_check))
            .service(get_cache)
            .service(post_cache)
            .service(delete_cache)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
