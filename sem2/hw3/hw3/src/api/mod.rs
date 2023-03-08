use crate::{
    model::{
        item::{Item, ItemWithCurrency},
        user::{Currency, User},
    },
    services::{exchange_monitor::ExchangeRateMonitor, mongodb::MongoRepository},
};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, Json, Path},
    HttpResponse, ResponseError,
};

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("couldn't create user")]
    UserCreationError,
    #[error("couldn't insert item")]
    ItemInsertError,
    #[error("unsupported currency")]
    UnsupportedCurrency,
    #[error("server error: {0}")]
    ServerError(String),
    #[error("user not found")]
    UserNotFound,
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::UserCreationError => StatusCode::UNPROCESSABLE_ENTITY,
            MyError::UnsupportedCurrency => StatusCode::UNPROCESSABLE_ENTITY,
            MyError::ItemInsertError => StatusCode::UNPROCESSABLE_ENTITY,
            MyError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::UserNotFound => StatusCode::UNAUTHORIZED,
        }
    }
}

type Username = String;

#[post("/register/{username}/{currency}")]
pub async fn register(
    repo: web::Data<MongoRepository>,
    info: Path<(Username, String)>,
) -> Result<String, MyError> {
    let (username, currency) = info.into_inner();

    let currency = match currency.as_str() {
        "eur" => Ok(Currency::Eur),
        "usd" => Ok(Currency::Usd),
        "rub" => Ok(Currency::Rub),
        _ => Err(MyError::UnsupportedCurrency),
    }?;

    let username_copy = username.clone();
    let user = User {
        username: username,
        preferred_currency: currency,
    };
    repo.insert_user(user)
        .await
        .map_err(|_e| MyError::UserCreationError)?;
    Ok(format!("user {} registered", username_copy))
}

#[post("/add_item/{name}/{price_in_euro}")]
pub async fn add_item(
    repo: web::Data<MongoRepository>,
    info: Path<(String, f64)>,
) -> Result<String, MyError> {
    let (name, price_in_euro) = info.into_inner();
    let item = Item {
        name,
        price_in_euro,
    };
    repo.insert_item(item)
        .await
        .map_err(|_e| MyError::ItemInsertError)?;
    Ok("item added".to_string())
}

#[get("/view_catalogue/{username}")]
pub async fn view_catalogue(
    repo: web::Data<MongoRepository>,
    ex_monitor: web::Data<ExchangeRateMonitor>,
    username: Path<Username>,
) -> Result<Json<Vec<ItemWithCurrency>>, MyError> {
    let username = username.into_inner(); // получаем логин из URL
    let user = repo // ищем пользователя в базе данных
        .get_user(username)
        .await // асинхронно :)
        .map_err(|e| MyError::ServerError(e.to_string()))? // обрабатываем ошибки
        .ok_or(MyError::UserNotFound)?; // если пользователь не найден, возвращаем соответствующее сообщение

    let exchange_rate = ex_monitor.get_exchange_rate().await; // асинхронно получаем текущий курс валют
    let multiplier = match user.preferred_currency {
        Currency::Eur => 1.0,
        Currency::Usd => exchange_rate.to_usd,
        Currency::Rub => exchange_rate.to_rub,
    };

    let items = repo // получаем список товаров из базы данных
        .get_items()
        .await
        .map_err(|e| MyError::ServerError(e.to_string()))?;
    let currency = user.preferred_currency;
    Ok(Json(
        items
            .into_iter()
            .map(|item| ItemWithCurrency {
                name: item.name,
                price: multiplier * item.price_in_euro, // отображаем стоимость товара в выбранной валюте
                currency: currency.clone(),
            })
            .collect(),
    ))
}
