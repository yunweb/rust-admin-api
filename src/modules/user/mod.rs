pub(crate) mod service;
pub mod model;
pub mod util;

use actix_web::{HttpResponse, Responder};

pub async fn aa() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

pub async fn bb() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}
