mod chapas;
mod response;

use actix_web::{error, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use chapas::{Config, Status, Source};
use response::{Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    folder: String,
}

async fn install(config: web::Json<Config>) -> HttpResponse {
    match serde_json::to_string_pretty(&config.0) {
        Ok(contents) => match Config::write(&config, contents) {
            Ok(_) => {
                Status::write(&config, "installing");

                match Source::install(&config) {
                    Ok(_) => {
                        Status::write(&config, "installed");
                        Source::init(&config).expect("something went wrong");
                        ()
                    }
                    Err(_) => ()
                }

                Response::success(format!("installed {}", String::from(&config.name)))
            }
            Err(err) => Response::error(err.to_string())
        },
        Err(err) => Response::error(err.to_string()),
    }
}

async fn build(project: web::Json<Project>) -> HttpResponse {
    println!("project input: {:?}", &project);

    /*
    Command::new("node")
        // TODO: can we predict error?
        .current_dir(format!("{}/{}", CHAPA_SOURCE, project.0.name))
        .arg("index.js")
        .spawn()
        .expect("build failed");
    */
    // TODO: handle errors properly
    /*
    Status::write(
        String::from("it is building"),
        serde_json::to_string(&project.0).expect("this error should be handled"),
    )
    .expect("nope");
     */

    Response::success(format!("building.."))
}

async fn status(name: web::Path<String>) -> HttpResponse {
    Response::success(format!("read status for {} from status file ", name.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/install", web::post().to(install))
            .route("/build", web::post().to(build))
            .route("/status/{name}", web::get().to(status))
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
