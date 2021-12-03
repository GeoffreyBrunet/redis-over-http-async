use crate::structs::Info;
use actix_web::{delete, get, post, web, web::Bytes, HttpResponse, Responder};
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;

type Pool = mobc::Pool<RedisConnectionManager>;

#[get("/cache/{key}")]
pub async fn get_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let s: String = conn
        .get(info.key.to_string())
        .await
        .unwrap_or(String::from("Key does not exist."));
    HttpResponse::Ok().body(s)
}

#[post("/cache/{key}")]
pub async fn post_cache(
    pool: web::Data<Pool>,
    info: web::Path<Info>,
    bytes: Bytes,
) -> impl Responder {
    let from_body: String = String::from_utf8(bytes.to_vec()).unwrap();
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.set(info.key.to_string(), from_body).await.unwrap();
    HttpResponse::Ok().body(s)
}

#[post("/cache/{key}/{ttl_value}")]
pub async fn post_cache_with_ttl(
    pool: web::Data<Pool>,
    info: web::Path<Info>,
    ttl_value: web::Path<usize>,
    bytes: Bytes,
) -> impl Responder {
    let from_body: String = String::from_utf8(bytes.to_vec()).unwrap();
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.set(info.key.to_string(), from_body).await.unwrap();
    let selected_key: String = conn.get(info.key.to_string()).await.unwrap();
    let s2: String = conn.expire(selected_key, ttl_value.unwrap()).await.unwrap();
    HttpResponse::Ok().body(format!("Key {} have {} ttl.", s, s2))
}

#[delete("/cache/{key}")]
pub async fn delete_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let s: isize = conn.del(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(s.to_string())
}
