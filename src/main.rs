use actix::{Actor, StreamHandler};
use actix_web::{App, HttpServer, web, HttpRequest, Error, HttpResponse};
use actix_web_actors::ws;

mod api;
mod router;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        match msg {
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        App::new()
        .service(router::navigation::route("/routes/navigation"))
        .service(api::health_service::route("/api/v1/health"))
        .route("/ws/", web::get().to(index))
        .service(actix_files::Files::new("/", "./public/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
