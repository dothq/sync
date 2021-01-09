use actix_web::{web, App, HttpServer};

use dotenv::dotenv;
use std::env;

mod commit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("SYNC_SERVER_PORT")
                .expect("SYNC_SERVER_PORT must be set");
    
    HttpServer::new(|| {
        App::new()
            .route("/commit", web::get().to(commit::main))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}