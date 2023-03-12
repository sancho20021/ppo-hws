use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{add_company, add_stock, buy, change_stock_price, get_price_and_number, sell};
use services::market::MutexedMarket;

mod api;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let market = MutexedMarket::new();

    HttpServer::new(move || {
        let market = Data::new(market.clone());
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(market)
            .service(add_company)
            .service(add_stock)
            .service(get_price_and_number)
            .service(buy)
            .service(sell)
            .service(change_stock_price)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
