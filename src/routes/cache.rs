use crate::structs::Info;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mobc_redis::redis::AsyncCommands;
use mobc_redis::RedisConnectionManager;
type Pool = mobc::Pool<RedisConnectionManager>;

#[get("/cache/{key}")]
pub async fn get_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    // TODO: return errors
    let mut conn = pool.get().await.unwrap();
    //HttpResponse::error("Key don't exists")
    let s: String = conn.get(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(s)
}

#[post("/cache/{key}")]
pub async fn post_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    // TODO: check for ttl -> expire function, and get data from body
    let value: String = String::from("bar");
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.set(info.key.to_string(), value).await.unwrap();
    HttpResponse::Ok().body(s)
}

#[delete("/cache/{key}")]
pub async fn delete_cache(pool: web::Data<Pool>, info: web::Path<Info>) -> impl Responder {
    // TODO: Response was of incompatible type: "Response type not string compatible." (response was int(1))'
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.del(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(s)
}
