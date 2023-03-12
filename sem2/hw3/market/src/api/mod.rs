use std::ops::DerefMut;

use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, Json, Path},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};

use crate::services::market::{Market, MarketError, MutexedMarket, StockPrice};

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("market error: {0}")]
    MarketError(MarketError),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

async fn with_locked_market_and_wrapped_error<F, T>(
    market: web::Data<MutexedMarket>,
    f: F,
) -> Result<T, MyError>
where
    F: FnOnce(&mut Market) -> Result<T, MarketError>,
{
    let mut market_mutex = market.market.lock().await;
    let market = market_mutex.deref_mut();
    f(market).map_err(|e| MyError::MarketError(e))
}

#[post("/add-company/{company}")]
pub async fn add_company(
    market: web::Data<MutexedMarket>,
    company: Path<String>,
) -> Result<String, MyError> {
    with_locked_market_and_wrapped_error(market, |market| {
        market
            .add_company(company.into_inner())
            .map(|()| "company added".to_string())
    })
    .await
}

#[post("/add-stock/{company}")]
pub async fn add_stock(
    market: web::Data<MutexedMarket>,
    company: Path<String>,
) -> Result<String, MyError> {
    let company = company.into_inner();
    with_locked_market_and_wrapped_error(market, |market| {
        market
            .add_stock(&company)
            .map(|()| "stock added".to_string())
    })
    .await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockView {
    company: String,
    number: usize,
    price: StockPrice,
}

#[get("/get-price-and-number/{company}")]
pub async fn get_price_and_number(
    market: web::Data<MutexedMarket>,
    company: Path<String>,
) -> Result<Json<StockView>, MyError> {
    let company = company.into_inner();
    with_locked_market_and_wrapped_error(market, |market| {
        let stock = market.get_stock(&company)?;
        Ok(Json(StockView {
            company,
            number: stock.number,
            price: stock.price,
        }))
    })
    .await
}

#[post("/buy/{company}/{price}")]
pub async fn buy(
    market: web::Data<MutexedMarket>,
    info: Path<(String, StockPrice)>,
) -> Result<String, MyError> {
    let (company, price) = info.into_inner();
    with_locked_market_and_wrapped_error(market, |market| {
        market
            .buy(&company, price)
            .map(|()| "stock bought".to_string())
    })
    .await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sold {
    price: StockPrice,
}

#[post("/sell/{company}")]
pub async fn sell(
    market: web::Data<MutexedMarket>,
    company: Path<String>,
) -> Result<Json<Sold>, MyError> {
    let company = company.into_inner();
    with_locked_market_and_wrapped_error(market, |market| {
        market.sell(&company).map(|price| Json(Sold { price }))
    })
    .await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceChanged {
    price: StockPrice,
}

#[post("/change-stock-price/{company}")]
pub async fn change_stock_price(
    market: web::Data<MutexedMarket>,
    company: Path<String>,
) -> Result<Json<PriceChanged>, MyError> {
    let company = company.into_inner();
    with_locked_market_and_wrapped_error(market, |market| {
        market
            .change_stock_price(&company)
            .map(|price| Json(PriceChanged { price }))
    })
    .await
}
