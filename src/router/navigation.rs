use actix_web::{post, web::scope, HttpResponse, Responder, Scope};

pub fn route(prefix: &str) -> Scope {
    return scope(prefix).service(index);
}

#[post("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from {:?} navigation", rand::random::<u32>()))
}
