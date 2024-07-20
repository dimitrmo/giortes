use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use futures_locks::RwLock;
use giortes_lib::Eortologio;
use log::{debug, info};
use std::sync::Arc;
use std::time::Duration;

#[get("/giortes")]
async fn giortes_handler(data: web::Data<Arc<RwLock<Eortologio>>>) -> impl Responder {
    let eortologio = data.read().await;
    let giortes = eortologio.get_giortes();
    let body = serde_json::to_string(giortes).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/version")]
async fn version_handler() -> impl Responder {
    env!("CARGO_PKG_VERSION")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port: String = match env::var_os("PORT") {
        Some(v) => {
            v.into_string().unwrap_or(String::from("8080"))
        }
        None => String::from("8080")
    };

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

    info!("starting server at :{port}");

    let data = web::Data::new(eortologio);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(giortes_handler)
            .service(version_handler)
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await
}
