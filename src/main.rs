extern crate chrono;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::middleware::Logger;
use actix_files as fs;
use serde::{Deserialize, Serialize};
use env_logger;

use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct DateTimeObj {
    utc: String,
    local: String, 
}    

async fn get_time() -> HttpResponse {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = DateTime::from(utc);
    HttpResponse::Ok().json(DateTimeObj {
        utc: utc.to_rfc3339(),
        local: local.to_rfc3339(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/time", web::get().to(get_time))
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
        /*
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
            .route("/time/", web::get().to(index))
        */
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
