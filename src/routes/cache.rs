use crate::structs::{Info, Infos};
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
    HttpResponse::Ok().body(format!("{} {}", info.key.to_string(), s))
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
    HttpResponse::Ok().body(format!("key: {}, value: {} are added.", info.key.to_string(), s))
}

#[post("/cache/{key}/{set_ttl}")]
pub async fn post_cache_with_ttl(
    pool: web::Data<Pool>,
    info: web::Path<Infos>,
    bytes: Bytes,
) -> impl Responder {
    let from_body: String = String::from_utf8(bytes.to_vec()).unwrap();
    let mut conn = pool.get().await.unwrap();
    let _s: String = conn.set(info.key.to_string(), from_body).await.unwrap();
    let _s2: usize = conn.expire(info.key.to_string(), info.set_ttl).await.unwrap();
    HttpResponse::Ok().body(format!("Key {} have {} ttl.", info.key, info.set_ttl))
}

#[delete("/cache/{key}")]
pub async fn delete_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let _s: isize = conn.del(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(format!("{} deleted.", info.key.to_string()))
}
