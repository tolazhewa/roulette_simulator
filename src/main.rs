use actix_web::{App, HttpServer};
use api::endpoints::run_with_files;

mod agent;
mod api;
mod bet;
mod board;
mod error;
mod json;
mod roulette;
mod types;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry::Registry};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (file_guard, json_file_guard) = set_up_logging();
    let _ = HttpServer::new(|| App::new().service(run_with_files))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    drop(file_guard);
    drop(json_file_guard);
    Ok(())
}

fn set_up_logging() -> (WorkerGuard, WorkerGuard) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "app.log");
    let json_file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "app.json");
    let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);
    let (non_blocking_json_file, json_file_guard) =
        tracing_appender::non_blocking(json_file_appender);

    let stdout_layer = fmt::layer().with_writer(|| std::io::stdout());
    let file_layer = fmt::layer().with_writer(move || non_blocking_file.clone());
    let json_file_layer = fmt::layer()
        .json()
        .with_writer(move || non_blocking_json_file.clone());

    let subscriber = Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(json_file_layer)
        .with(LevelFilter::DEBUG);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global tracing subscriber");

    (file_guard, json_file_guard)
}
