use actix_web::{get, post, delete, web, HttpResponse, Responder};
use mobc_redis::RedisConnectionManager;
use mobc_redis::redis::AsyncCommands;
use crate::structs::Info;
type Pool = mobc::Pool<RedisConnectionManager>;

#[get("/cache/{key}")]
pub async fn get_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    // TODO: return errors
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.get(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(s)
}

#[post("/cache/{key}")]
pub async fn post_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    let value: String = String::from("bar");
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.set(info.key.to_string(), value).await.unwrap();
    HttpResponse::Ok().body(s)
}

#[delete("/cache/{key}")]
pub async fn delete_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.del(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(s)
}

