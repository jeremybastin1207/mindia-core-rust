use actix_web::{get, web, HttpResponse, Responder};

use crate::api::app_state::AppState;

#[get("/transformation/templates")]
pub async fn get_transformation_templates(data: web::Data<AppState>) -> impl Responder {
    let transformation_templates = data.transformation_template_registry.get_all();

    HttpResponse::Ok().json(transformation_templates)
}
