use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::api::app_state::AppState;
use crate::named_transformation::NamedTransformation;

#[get("/named_transformation")]
pub async fn get_named_transformations(data: web::Data<AppState>) -> impl Responder {
    let mut named_transformation_storage = data.named_transformation_storage.lock().unwrap();

    match named_transformation_storage.get_all() {
        Ok(named_transformations) => {
            let named_transformations: Vec<NamedTransformation> =
                named_transformations.into_iter().map(|(_, v)| v).collect();
            HttpResponse::Ok().json(named_transformations)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/named_transformation")]
pub async fn save_named_transformation(
    data: web::Data<AppState>,
    new_named_transformation: web::Json<NamedTransformation>,
) -> impl Responder {
    let mut named_transformation_storage = data.named_transformation_storage.lock().unwrap();

    let named_transformation = NamedTransformation {
        name: new_named_transformation.name.clone(),
        transformations: new_named_transformation.transformations.clone(),
    };

    match named_transformation_storage.save(named_transformation) {
        Ok(()) => HttpResponse::Ok().body(format!(
            "Named transformation {} saved",
            new_named_transformation.name
        )),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/named_transformation/{name}")]
pub async fn delete_named_transformation(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    let mut named_transformation_storage = data.named_transformation_storage.lock().unwrap();

    match named_transformation_storage.delete(&name) {
        Ok(()) => HttpResponse::Ok().body(format!("Named transformation {} deleted", name)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
