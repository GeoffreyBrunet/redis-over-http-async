use crate::structs::{HInfo, HInfos};
use actix_web::{delete, get, post, web, web::Bytes, HttpResponse, Responder};
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;

type Pool = mobc::Pool<RedisConnectionManager>;

#[get("/hashcache/{userid}/{key}")]
pub async fn get_hash_cache(pool: web::Data<Pool>, info: web::Path<HInfo>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let s: String = conn
        .hget(info.key.to_string(), info.userid.to_string())
        .await
        .unwrap_or(String::from("Key does not exist."));
    HttpResponse::Ok().body(format!("{} {}", info.key.to_string(), s))
}

#[post("/hashcache/{userid}/{key}")]
pub async fn post_hash_cache(
    pool: web::Data<Pool>,
    info: web::Path<HInfo>,
    bytes: Bytes,
) -> impl Responder {
    let from_body: String = String::from_utf8(bytes.to_vec()).unwrap();
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.hset(info.key.to_string(), info.userid.to_string(), from_body).await.unwrap();
    HttpResponse::Ok().body(format!("key: {}, value: {} are added.", info.key.to_string(), s))
}

#[post("/hashcache/{userid}/{key}/{set_ttl}")]
pub async fn post_hash_cache_with_ttl(
    pool: web::Data<Pool>,
    info: web::Path<HInfos>,
    bytes: Bytes,
) -> impl Responder {
    let from_body: String = String::from_utf8(bytes.to_vec()).unwrap();
    let mut conn = pool.get().await.unwrap();
    let _s: String = conn.hset(info.key.to_string(), info.userid.to_string(), from_body).await.unwrap();
    let _s2: usize = conn.expire(info.key.to_string(), info.set_ttl).await.unwrap();
    HttpResponse::Ok().body(format!("Key {} have {} ttl.", info.key, info.set_ttl))
}

#[delete("/hashcache/{userid}/{key}")]
pub async fn delete_hash_cache(pool: web::Data<Pool>, info: web::Path<HInfo>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let _s: isize = conn.hdel(info.key.to_string(), info.userid.to_string()).await.unwrap();
    HttpResponse::Ok().body(format!("{} deleted.", info.key.to_string()))
}
