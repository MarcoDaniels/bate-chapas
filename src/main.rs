mod chapas;

use actix_web::{error, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use chapas::{Config, Source, Status};

async fn install(config: web::Json<Config>) -> HttpResponse {
    match serde_json::to_string_pretty(&config.0) {
        Ok(contents) => match Config::write(&config, contents) {
            Ok(_) => {
                Status::write(&config, "installing");

                match Source::install(&config) {
                    Ok(_) => {
                        Status::write(&config, "installed");
                        Source::init(&config).expect("something went wrong");

                        HttpResponse::Ok().json(Status {
                            message: format!("installed {}", String::from(&config.name)),
                        })
                    }
                    Err(err) => HttpResponse::NotAcceptable().json(Status {
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => HttpResponse::NotAcceptable().json(Status {
                message: err.to_string(),
            }),
        },
        Err(err) => HttpResponse::BadRequest().json(Status {
            message: err.to_string(),
        }),
    }
}

async fn run(name: web::Path<String>) -> HttpResponse {
    match Config::read(&name.0) {
        Ok(contents) => {
            match Source::run(&contents) {
                Ok(_) => println!("it runs"),
                Err(err) => println!("it does not run: {}", err.to_string()),
            }

            HttpResponse::Ok().json(Status {
                message: format!("run script for {}", &contents.name),
            })
        }
        Err(_) => HttpResponse::NotFound().json(Status {
            message: format!("no configuration found for {}", name),
        }),
    }
}

async fn status(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(Status {
        message: format!("read status for {} from status file ", name.0),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/install", web::post().to(install))
            .route("/run/{name}", web::post().to(run))
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
