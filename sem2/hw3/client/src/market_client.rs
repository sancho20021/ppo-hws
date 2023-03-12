use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct MarketClient {
    client: Client,
    server_address: String,
}

type StockPrice = usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct StockView {
    pub company: String,
    pub number: usize,
    pub price: StockPrice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sold {
    pub price: StockPrice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceChanged {
    pub price: StockPrice,
}

async fn send_and_get_json<T: DeserializeOwned>(req: RequestBuilder) -> Result<T, String> {
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    match status {
        StatusCode::OK => (resp.json::<T>().await).map_err(|e| e.to_string()),
        _ => {
            let text = resp.text().await.map_err(|e| e.to_string())?;
            Err(format!("Error, status code: {}\n{}", status, text))
        }
    }
}

async fn send_and_get_text_response(req: RequestBuilder) -> Result<String, String> {
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    if status == StatusCode::OK {
        Ok(text)
    } else {
        Err(format!("Error, status code: {}\n{}", status, text))
    }
}

impl MarketClient {
    pub fn new(server_address: String) -> Self {
        Self {
            client: Client::new(),
            server_address,
        }
    }

    pub async fn add_company(&self, company: String) -> Result<(), String> {
        send_and_get_text_response(
            self.client
                .post(self.path(format!("/add-company/{company}"))),
        )
        .await
        .map(|_| ())
    }

    pub async fn add_stock(&self, company: String) -> Result<(), String> {
        send_and_get_text_response(self.client.post(self.path(format!("/add-stock/{company}"))))
            .await
            .map(|_| ())
    }

    pub async fn get_price_and_number(&self, company: String) -> Result<StockView, String> {
        send_and_get_json(
            self.client
                .get(self.path(format!("/get-price-and-number/{company}"))),
        )
        .await
    }

    pub async fn buy(&self, company: String, price: StockPrice) -> Result<(), String> {
        send_and_get_text_response(
            self.client
                .post(self.path(format!("/buy/{company}/{price}"))),
        )
        .await
        .map(|_| ())
    }

    pub async fn sell(&self, company: String) -> Result<Sold, String> {
        send_and_get_json(self.client.post(self.path(format!("/sell/{company}")))).await
    }

    pub async fn change_stock_price(&self, company: String) -> Result<PriceChanged, String> {
        send_and_get_json(
            self.client
                .post(self.path(format!("/change-stock-price/{company}"))),
        )
        .await
    }

    fn path(&self, path: String) -> String {
        let path = format!("{0}{path}", self.server_address);
        path
    }
}
