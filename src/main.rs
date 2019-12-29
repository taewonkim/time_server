extern crate chrono;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;

fn index() -> impl Responder {
    let dt: DateTime<Utc> = Utc::now();
    let fmt: String = dt.to_rfc3339();
    HttpResponse::Ok().body(fmt)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:5000")
    .unwrap()
    .run()
    .unwrap();
}
