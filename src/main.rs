use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use futures_locks::RwLock;
use giortes_lib::Eortologio;
use log::{debug, info};
use std::sync::Arc;
use std::time::Duration;

#[get("/giortes")]
async fn giortes_handler(data: web::Data<Arc<RwLock<Eortologio>>>) -> impl Responder {
    let eortologio = data.try_read().unwrap();
    let giortes = eortologio.get_giortes();
    let body = serde_json::to_string(giortes).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/version")]
async fn version_handler() -> impl Responder {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    VERSION
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let eortologio = Arc::new(RwLock::new(Eortologio::default()));

    let eortologio_shared = eortologio.clone();
    tokio::spawn(async move {
        let shared_eortologio = eortologio_shared.clone();
        loop {
            let mut eortologio = shared_eortologio.try_write().unwrap();
            let giortes = eortologio.refresh_giortes_async().await;
            debug!("giortes updated [{:?}]", giortes);
            drop(eortologio); // unlock
            tokio::time::sleep(Duration::from_secs(1200)).await; // every 20 minutes
        }
    });

    info!("starting server at :8080");

    let data = web::Data::new(eortologio);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(giortes_handler)
            .service(version_handler)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
