use std::future::Future;

use client::market_client::{self, MarketClient};
use testcontainers::clients;

use crate::market::Market;

mod market {
    use std::time::Duration;

    use testcontainers::{core::WaitFor, Image};

    #[derive(Debug, Default)]
    pub struct Market;

    impl Market {
        pub fn get_address(port: u16) -> String {
            format!("http://127.0.0.1:{port}")
        }
    }

    impl Image for Market {
        type Args = ();

        fn name(&self) -> String {
            "market".to_string()
        }

        fn tag(&self) -> String {
            "latest".to_string()
        }

        fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
            vec![WaitFor::Duration {
                length: Duration::from_millis(1),
            }]
        }
    }

    pub const SERVER_PORT: u16 = 8080;
}

async fn with_fresh_server<F, Fut>(test: F)
where
    Fut: Future<Output = ()>,
    F: FnOnce(MarketClient) -> Fut,
{
    let docker = clients::Cli::default();
    let container = docker.run(Market);
    let port = container.get_host_port_ipv4(market::SERVER_PORT);
    let client = market_client::MarketClient::new(Market::get_address(port));
    test(client).await;
}

fn company_a() -> String {
    "a".to_string()
}

#[tokio::test]
async fn test_add_company_ok() {
    with_fresh_server(|client| async move {
        client.add_company("hello".to_string()).await.unwrap();
    })
    .await;
}

#[tokio::test]
async fn test_create_company_twice() {
    with_fresh_server(|client| async move {
        client.add_company("hello".to_string()).await.unwrap();
        client.add_company("hello".to_string()).await.expect_err("");
    })
    .await;
}

#[tokio::test]
async fn add_get_stock_ok() {
    with_fresh_server(|client| async move {
        client.add_company(company_a()).await.unwrap();
        client.add_stock(company_a()).await.unwrap();
        let stock = client.get_price_and_number(company_a()).await.unwrap();
        assert_eq!(stock.company, company_a());
        assert_eq!(stock.number, 1);
    })
    .await;
}

#[tokio::test]
async fn add_stock_unknown_company() {
    with_fresh_server(|client| async move {
        client
            .get_price_and_number(company_a())
            .await
            .expect_err("");
    })
    .await;
}

#[tokio::test]
async fn buy_sell_stock_ok() {
    with_fresh_server(|client| async move {
        client.add_company(company_a()).await.unwrap();
        client.add_stock(company_a()).await.unwrap();
        let price = client
            .get_price_and_number(company_a())
            .await
            .unwrap()
            .price;
        client.buy(company_a(), price).await.unwrap();
        let price_sold = client.sell(company_a()).await.unwrap().price;
        assert_eq!(price, price_sold);
    })
    .await;
}

#[tokio::test]
async fn buy_no_stocks_left() {
    with_fresh_server(|client| async move {
        client.add_company(company_a()).await.unwrap();
        let price = client
            .get_price_and_number(company_a())
            .await
            .unwrap()
            .price;
        client.buy(company_a(), price).await.expect_err("");
    })
    .await;
}

#[tokio::test]
async fn buy_wrong_price() {
    with_fresh_server(|client| async move {
        client.add_company(company_a()).await.unwrap();
        client.add_stock(company_a()).await.unwrap();
        let price = client
            .get_price_and_number(company_a())
            .await
            .unwrap()
            .price;
        client.buy(company_a(), price + 1).await.expect_err("");
    })
    .await;
}

#[tokio::test]
async fn sell_at_changed_price_ok() {
    with_fresh_server(|client| async move {
        client.add_company(company_a()).await.unwrap();
        client.add_stock(company_a()).await.unwrap();
        let price = client
            .get_price_and_number(company_a())
            .await
            .unwrap()
            .price;
        client.buy(company_a(), price).await.unwrap();

        let new_price = client.change_stock_price(company_a()).await.unwrap().price;
        let price_sold = client.sell(company_a()).await.unwrap().price;
        assert_eq!(new_price, price_sold);
    })
    .await;
}
