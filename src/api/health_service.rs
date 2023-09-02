use actix_web::{get, web::scope, HttpResponse, Responder, Scope};

pub fn route(prefix: &str) -> Scope {
    return scope(prefix).service(alive);
}

#[get("/alive")]
async fn alive() -> impl Responder {
    HttpResponse::Ok().body("alive")
}
