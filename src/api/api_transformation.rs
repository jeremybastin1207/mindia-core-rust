use actix_web::{get, web, HttpResponse, Responder};

use crate::api::app_state::AppState;

#[get("/transformation-descriptions")]
pub async fn get_transformation_descriptions(data: web::Data<AppState>) -> impl Responder {
    let transformation_description_registry =
        data.transformation_description_registry.lock().unwrap();

    let transformation_descriptions = transformation_description_registry.get_all();

    HttpResponse::Ok().json(transformation_descriptions)
}
