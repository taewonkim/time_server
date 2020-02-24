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

fn _get_time() -> DateTimeObj {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = DateTime::from(utc);
    DateTimeObj {
        utc: utc.to_rfc3339(),
        local: local.to_rfc3339(),
    }
}

async fn get_time() -> HttpResponse {
    HttpResponse::Ok().json(_get_time())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Start server...");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/time", web::get().to(get_time))
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
