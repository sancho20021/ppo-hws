use std::collections::HashMap;

use futures::lock::Mutex;
use rand::Rng;

pub type StockPrice = usize;
const DEFAULT_PRICE: StockPrice = 10;

#[derive(Debug, thiserror::Error)]
pub enum MarketError {
    #[error("company with given name already exists")]
    CompanyAlreadyExists,
    #[error("company with given name not found")]
    CompanyNotFound,
    #[error("no stocks left")]
    NoStocksLeft,
    #[error("suggested and real stock prices don't match")]
    WrongPrice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stock {
    pub price: StockPrice,
    pub number: usize,
}

pub struct Market {
    stocks: HashMap<String, Stock>,
}

pub struct MutexedMarket {
    pub market: Mutex<Market>,
}

impl Market {
    pub fn new() -> Self {
        Self {
            stocks: Default::default(),
        }
    }

    pub fn add_company(&mut self, company: String) -> Result<(), MarketError> {
        if self.stocks.contains_key(&company) {
            Err(MarketError::CompanyAlreadyExists)
        } else {
            self.stocks.insert(
                company,
                Stock {
                    price: DEFAULT_PRICE,
                    number: 0,
                },
            );
            Ok(())
        }
    }

    pub fn add_stock(&mut self, company: &String) -> Result<(), MarketError> {
        let mut stock = self
            .stocks
            .get_mut(company)
            .ok_or(MarketError::CompanyNotFound)?;
        stock.number += 1;
        Ok(())
    }

    pub fn get_stock(&self, company: &String) -> Result<Stock, MarketError> {
        self.stocks
            .get(company)
            .ok_or(MarketError::CompanyNotFound)
            .cloned()
    }

    pub fn buy(&mut self, company: &String, price: StockPrice) -> Result<(), MarketError> {
        let mut stock = self
            .stocks
            .get_mut(company)
            .ok_or(MarketError::CompanyNotFound)?;
        if stock.price == price {
            if stock.number > 0 {
                stock.number -= 1;
                Ok(())
            } else {
                Err(MarketError::NoStocksLeft)
            }
        } else {
            Err(MarketError::WrongPrice)
        }
    }

    pub fn sell(&mut self, company: &String) -> Result<StockPrice, MarketError> {
        let mut stock = self
            .stocks
            .get_mut(company)
            .ok_or(MarketError::CompanyNotFound)?;
        stock.number += 1;
        Ok(stock.price)
    }

    pub fn change_stock_price(&mut self, company: &String) -> Result<StockPrice, MarketError> {
        let mut stock = self
            .stocks
            .get_mut(company)
            .ok_or(MarketError::CompanyNotFound)?;
        let new_price = Self::random_price();
        stock.price = new_price;
        Ok(new_price)
    }

    fn random_price() -> StockPrice {
        rand::thread_rng().gen_range(1..100)
    }
}

#[cfg(test)]
mod tests {
    use super::{Market, Stock, DEFAULT_PRICE};

    fn company_a() -> String {
        "A".to_string()
    }

    #[test]
    pub fn test_create_company_ok() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
    }

    #[test]
    pub fn test_create_company_twice() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
        market.add_company(company_a()).expect_err("");
    }

    #[test]
    pub fn add_get_stock_ok() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
        market.add_stock(&company_a()).unwrap();
        let stock = market.get_stock(&company_a()).unwrap();
        assert_eq!(
            stock,
            Stock {
                price: DEFAULT_PRICE,
                number: 1
            }
        );
    }

    #[test]
    pub fn add_stock_unknown_company() {
        let mut market = Market::new();
        market.add_stock(&company_a()).expect_err("");
    }

    #[test]
    pub fn buy_sell_stock_ok() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
        market.add_stock(&company_a()).unwrap();
        market.buy(&company_a(), DEFAULT_PRICE).unwrap();
        let price = market.sell(&company_a()).unwrap();
        assert_eq!(price, DEFAULT_PRICE);
    }

    #[test]
    pub fn buy_no_stocks_left() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
        market.buy(&company_a(), DEFAULT_PRICE).expect_err("");
    }

    #[test]
    pub fn buy_wrong_price() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
        market.add_stock(&company_a()).unwrap();
        market.buy(&company_a(), DEFAULT_PRICE + 1).expect_err("");
    }

    #[test]
    pub fn sell_at_changed_price_ok() {
        let mut market = Market::new();
        market.add_company(company_a()).unwrap();
        market.add_stock(&company_a()).unwrap();
        market.buy(&company_a(), DEFAULT_PRICE).unwrap();

        let new_price = market.change_stock_price(&company_a()).unwrap();
        let price = market.sell(&company_a()).unwrap();
        assert_eq!(price, new_price);
    }
}
