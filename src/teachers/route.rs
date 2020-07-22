use super::model::{Login, Teacher, Teachers};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

type ResponseType = Result<HttpResponse, CustomError>;

#[derive(Deserialize)]
pub struct Info {
    department: String,
}

#[post("/teacher/login")]
async fn login(user: web::Json<Login>) -> ResponseType {
    let response = Teachers::login(user.into_inner())?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/teachers")]
async fn find_by_department(info: web::Query<Info>) -> ResponseType {
    let teachers = Teachers::find_by_department(info.into_inner().department)?;
    Ok(HttpResponse::Ok().json(teachers))
}

#[get("/all_teachers")]
async fn find_all() -> ResponseType {
    let teachers = Teachers::find_all()?;
    Ok(HttpResponse::Ok().json(teachers))
}

#[get("/teachers/{id}")]
async fn find(id: web::Path<i32>) -> ResponseType {
    let teacher = Teachers::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(teacher))
}

#[post("/teachers")]
async fn create(teacher: web::Json<Teacher>) -> ResponseType {
    let teacher = Teachers::create(teacher.into_inner())?;
    Ok(HttpResponse::Ok().json(teacher))
}

#[put("/teachers/{id}")]
async fn update(id: web::Path<i32>, teacher: web::Json<Teacher>) -> ResponseType {
    let teacher = Teachers::update(id.into_inner(), teacher.into_inner())?;
    Ok(HttpResponse::Ok().json(teacher))
}

#[delete("/teachers/{id}")]
async fn delete(id: web::Path<i32>) -> ResponseType {
    let deleted_teacher = Teachers::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_teacher })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(login);
    config.service(find_by_department);
    config.service(create);
    config.service(update);
    config.service(delete);
}
