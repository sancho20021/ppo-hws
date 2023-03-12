use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{add_item, register, view_catalogue};
use services::{exchange_monitor::ExchangeRateMonitor, mongodb::MongoRepository};

mod api;
mod model;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let repo = MongoRepository::new("mongodb://localhost:27017")
        .await
        .expect("database error");
    let exchange_monitor = ExchangeRateMonitor;

    HttpServer::new(move || {
        let mongodb_data = Data::new(repo.clone());
        let ex_monitor_data = Data::new(exchange_monitor.clone());
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(mongodb_data)
            .app_data(ex_monitor_data)
            .service(register)
            .service(add_item)
            .service(view_catalogue)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
