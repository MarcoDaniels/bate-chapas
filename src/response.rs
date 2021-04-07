use actix_web::HttpResponse;
use crate::chapas::Status;

pub struct Response;

impl Response {
    pub fn success(message: String) -> HttpResponse {
        HttpResponse::Ok().json(Status { message })
    }

    pub fn error(message: String) -> HttpResponse {
        HttpResponse::BadRequest().json(Status { message })
    }
}