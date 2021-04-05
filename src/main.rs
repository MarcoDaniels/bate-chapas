use actix_web::{error, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

// const CHAPA_CONFIG: &str = "chapas/config";
const CHAPA_SOURCE: &str = "chapas/source";
const CHAPA_STATUS: &str = "chapas/status";

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    folder: String,
}

#[derive(Serialize, Deserialize)]
struct Status {
    status: String,
}

fn update_status_file(data: String, project: Project) {
    fs::write(format!("{}/{}.txt", CHAPA_STATUS, project.name), data)
        .expect(&*format!("Could not update status for {}", project.name))
}

async fn build(project: web::Json<Project>) -> HttpResponse {
    println!("project input: {:?}", &project);

    Command::new("node")
        // TODO: can we predict error?
        .current_dir(format!("{}/{}", CHAPA_SOURCE, project.0.name))
        .arg("index.js")
        .spawn()
        .expect("build failed");

    update_status_file(String::from("it is building"), project.0);

    HttpResponse::Ok().json(Status {
        status: String::from("building..."),
    })
}


async fn status(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(Status {
        status: String::from(format!("read status for {} from status file ", name.0)),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // .route("/install", web::post().to(install))
            .route("/build", web::post().to(build))
            .route("/status/{name}",web::get().to(status))
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}