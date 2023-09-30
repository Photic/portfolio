use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use handlebars::Handlebars;
use serde::Serialize;

use log::debug;
use serde_json::json;

use crate::model::template_names::TemplateName;

fn conditional_render<T: Serialize>(
    request: &HttpRequest,
    hb: Data<Handlebars<'_>>,
    partial_name: &str,
    context: &T,
) -> HttpResponse {
    let partial = match hb.render(partial_name, context) {
        Ok(partial) => partial,
        Err(e) => {
            debug!("Failed to render partial: {}", e);
            match page_not_found(hb) {
                Ok(body) => return HttpResponse::NotFound().body(body),
                Err(_) => {
                    debug!("Failed to render not_found");
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
    };

    if request.headers().get("hx-target").is_some() {
        debug!("Partial request detected, returning partial.");
        HttpResponse::Ok().body(partial)
    } else {
        debug!("Full request detected, returning full page.");
        match hb.render(
            TemplateName::layout.as_str(),
            &json!({
                "content": partial
            }),
        ) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(e) => {
                println!("Failed to render layout: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
}

pub async fn index(hb: web::Data<Handlebars<'_>>, request: HttpRequest) -> HttpResponse {
    conditional_render(&request, hb, TemplateName::home.as_str(), &json!({}))
}

pub async fn default_page_navigation(
    hb: web::Data<Handlebars<'_>>,
    request: HttpRequest,
) -> HttpResponse {
    let page_name = request
        .match_info()
        .get("page_name")
        .unwrap_or(TemplateName::home.as_str());
    conditional_render(&request, hb, page_name, &json!({}))
}

// Catch all 404 page start.

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
    match hb.render(TemplateName::not_found.as_str(), &json!({})) {
        Ok(body) => Ok(body),
        Err(e) => {
            println!("Failed to render not_found: {}", e);
            return Err(HttpResponse::InternalServerError().finish());
        }
    }
}

// Cach all 404 page end.
