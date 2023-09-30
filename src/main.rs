use actix::{Actor, StreamHandler};
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use handlebars::Handlebars;

mod app;
mod model;

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

async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    resp
}

#[get("/api/v1/health")]
async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("OK"))
}

// async fn generate_template_enum_file() {
//     // Get page names from pages folder
//     let file_names = std::fs::read_dir("./src/pages")
//         .unwrap()
//         .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
//         .collect::<Result<Vec<_>, std::io::Error>>()
//         .unwrap();

//     let mut page_names = Vec::new();

//     for file_name in file_names {
//         if file_name.ends_with(".html") {
//             page_names.push(file_name.replace(".html", ""));
//         }
//     }

//     let enum_variants = page_names
//         .iter()
//         .map(|file| format!("{},", file))
//         .collect::<Vec<_>>()
//         .join("\n");

//     let as_str_variants = page_names
//         .iter()
//         .map(|file| {
//             format!(
//                 r#"
//             TemplateName::{} => "{}","#,
//                 file, file
//             )
//         })
//         .collect::<Vec<_>>()
//         .join("\n");

//     let code = format!(
//         r#"
//         #![allow(dead_code, unused_variables)] 
//         #[allow(non_camel_case_types)]
//         #[derive(Debug)]
//         pub enum TemplateName {{
//             {}
//         }}

//         impl TemplateName {{
//             pub fn as_str(&self) -> &'static str {{
//                 match self {{
//                     {}
//                 }}
//             }}
//         }}"#,
//         enum_variants, as_str_variants
//     );

//     match std::fs::write("./src/generated/template_names.rs", code) {
//         Ok(_) => (),
//         Err(e) => println!("Failed to generate template names: {}", e),
//     };
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    // Create a Handlebars instance and register templates.
    let mut hb = Handlebars::new();

    hb.register_templates_directory(".html", "./src/pages")
        .expect("Failed to register templates directory.");

    let hb_state = web::Data::new(hb);

    // if env::var("environment").unwrap_or_else(|_| "prod".to_string()) == "dev" {
    //     tokio::task::spawn(generate_template_enum_file());
    // }

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(hb_state.clone())
            .service(health)
            .route("/ws/", web::get().to(websocket))
            .service(actix_files::Files::new("/static", "static"))
            .service(web::resource("/").route(web::get().to(app::index)))
            .service(
                web::resource("/{page_name}").route(web::get().to(app::default_page_navigation)),
            )
            // Catch all 404 page.
            .default_service(web::route().to(app::not_found))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
