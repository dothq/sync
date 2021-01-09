use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Commit {
    ok: bool
}

pub async fn main() -> impl Responder {
    HttpResponse::Ok().json(Commit {
        ok: true
    })
}