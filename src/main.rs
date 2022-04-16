use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use giortes_lib::{Eortologio, Giortes};
use log::{debug, info, warn};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use tokio_cron_scheduler::{Job, JobScheduler};

fn setup_cron(data: Arc<Eortologio>) -> JobScheduler {
    let sched = JobScheduler::new().unwrap();

    if let Err(e) = sched.add(
        Job::new_async("0 1,58 * * * *", move |_uuid, _l| {
            // let mut data = data.clone();
            // let data = data.clone();
            Box::pin(async move {
                let mut eortologio = Arc::clone(data).as_ref();
                let giortes = eortologio.fetch_giortes().await;
                eortologio.set_giortes(Box::new(giortes));
            })
        })
        .unwrap(),
    ) {
        warn!("error scheduling {:?}", e);
    }

    sched
}

#[get("/giortes")]
async fn giortes_handler(data: web::Data<Arc<Mutex<Eortologio>>>) -> impl Responder {
    let eortologio = data.lock().unwrap();
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
    let mut eortologio = Eortologio::default();
    let giortes = eortologio.fetch_giortes().await;
    debug!("giortes arrived {:?}", giortes);
    eortologio.set_giortes(Box::new(giortes));

    let eortologio_mut = Arc::new(eortologio);

    let scheduler = setup_cron(eortologio_mut.clone());
    if let Err(e) = scheduler.start() {
        warn!("scheduler error {:?}", e);
    } else {
        info!("scheduler started");
    }

    info!("starting server at :8080");

    let data = web::Data::new(eortologio_mut);

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
