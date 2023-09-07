use actix_web::{
    web::{self},
    HttpRequest, HttpResponse,
};

use handlebars::Handlebars;
use std::collections::BTreeMap;

use log::debug;

fn conditional_render(
    request: &HttpRequest,
    hb: web::Data<Handlebars<'_>>,
    partial_name: &str,
    context: &BTreeMap::<&str, &str>,
) -> HttpResponse {
    let partial = match hb.render(partial_name, context) {
        Ok(partial) => partial,
        Err(e) => {
            println!("Failed to render partial: {}", e);
            match page_not_found(hb) {
                Ok(body) => return HttpResponse::NotFound().body(body),
                Err(_) => {
                    println!("Failed to render not_found");
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
    };

    if request.headers().get("hx-target").is_some() {
        debug!("Partial request detected, returning partial.");
        HttpResponse::Ok().body(partial)
    } else {
        let mut context: BTreeMap<&str, &str> = BTreeMap::new();
        context.insert("content", partial.as_str());

        let body = match hb.render("index", &context) {
            Ok(body) => body,
            Err(e) => {
                println!("Failed to render index: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        debug!("Full request detected, returning full page.");
        HttpResponse::Ok().body(body)
    }
}

pub async fn index(hb: web::Data<Handlebars<'_>>, request: HttpRequest) -> HttpResponse {
    conditional_render(&request, hb, "home", &BTreeMap::<&str, &str>::new())
}

pub async fn default_page_navigation(hb: web::Data<Handlebars<'_>>, request: HttpRequest) -> HttpResponse {
    let page_name = request.match_info().get("page_name").unwrap_or("index");
    conditional_render(&request, hb, page_name, &BTreeMap::<&str, &str>::new())
}

// Catch all 404 page.

pub async fn not_found(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    match page_not_found(hb) {
        Ok(body) => return HttpResponse::NotFound().body(body),
        Err(_) => {
            println!("Failed to render not_found");
            return HttpResponse::InternalServerError().finish();
        }
    };
}

fn page_not_found(hb: web::Data<Handlebars<'_>>) -> Result<String, HttpResponse> {
    match hb.render("not_found", &BTreeMap::<&str, &str>::new()) {
        Ok(body) => Ok(body),
        Err(e) => {
            println!("Failed to render not_found: {}", e);
            return Err(HttpResponse::InternalServerError().finish());
        }
    }
}
