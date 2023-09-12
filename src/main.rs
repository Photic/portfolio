use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Result, middleware};
use actix_web_actors::ws;
use handlebars::Handlebars;

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

    // Create a Handlebars instance and register templates.
    let mut hb = Handlebars::new();

    hb.register_templates_directory(".hbs", "./static/pages")
        .expect("Failed to register templates directory.");

    hb.register_templates_directory(".hbs", "./static/components")
        .expect("Failed to register components directory.");

    let hb_state = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(hb_state.clone())
            .service(api::health_service::route("/api/v1/health"))
            .route("/ws/", web::get().to(index))
            .service(actix_files::Files::new("/static", "static"))
            .service(web::resource("/").route(web::get().to(router::navigation::index)))
            .service(
                web::resource("/{page_name}")
                    .route(web::get().to(router::navigation::default_page_navigation)),
            )
            // Catch all 404 page.
            .default_service(web::route().to(router::navigation::not_found))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
