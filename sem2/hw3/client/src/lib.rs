use std::collections::HashMap;

use market_client::{MarketClient, Sold, StockPrice};

pub mod market_client;

pub struct PersonalStockView {
    pub company: String,
    pub price: StockPrice,
    pub number: usize,
}

#[derive(Default)]
pub struct UserState {
    balance: usize,
    stocks: HashMap<String, usize>,
}

pub type UserId = String;

/// Вторая часть системы для дз.
/// Личный кабинет пользователя.
pub struct AccountService {
    users: HashMap<UserId, UserState>,
    market_client: MarketClient,
}

impl AccountService {
    pub fn add_user(&mut self, id: UserId) -> Result<(), String> {
        if self.users.contains_key(&id) {
            Err("user already exists".to_string())
        } else {
            self.users.insert(id, Default::default());
            Ok(())
        }
    }

    pub fn replenish(&mut self, id: UserId, amount: usize) -> Result<(), String> {
        match self.users.get_mut(&id) {
            Some(state) => {
                state.balance += amount;
                Ok(())
            }
            None => Err("user not found".to_string()),
        }
    }

    pub async fn view_stocks(&self, id: UserId) -> Result<Vec<PersonalStockView>, String> {
        match self.users.get(&id) {
            Some(state) => {
                let stocks = state.stocks.iter();
                let views = stocks
                    .map(|(company, number)| async move {
                        let price = self
                            .market_client
                            .get_price_and_number(company.clone())
                            .await;
                        match price {
                            Ok(stock_view) => Ok(PersonalStockView {
                                company: company.clone(),
                                price: stock_view.price,
                                number: number.clone(),
                            }),
                            Err(e) => Err(e),
                        }
                    })
                    .collect::<Vec<_>>();
                futures::future::join_all(views)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
            }
            None => Err("user not found".to_string()),
        }
    }

    pub async fn get_total_stocks_value(&self, id: UserId) -> Result<StockPrice, String> {
        let stocks = self.view_stocks(id).await?;
        Ok(stocks
            .iter()
            .fold(0, |sum, view| sum + view.price * view.number))
    }

    pub async fn buy_stock(&mut self, id: UserId, company: String) -> Result<(), String> {
        match self.users.get_mut(&id) {
            Some(state) => {
                let price = self
                    .market_client
                    .get_price_and_number(company.clone())
                    .await?
                    .price;
                if state.balance < price {
                    return Err("Not enough money".to_string());
                }

                self.market_client.buy(company.clone(), price).await?;
                state.balance -= price;

                match state.stocks.get_mut(&company.clone()) {
                    Some(number) => *number += 1,
                    None => {
                        state.stocks.insert(company, 1);
                    }
                }
                Ok(())
            }
            None => Err("user not found".to_string()),
        }
    }

    pub async fn sell_stock(&mut self, id: UserId, company: String) -> Result<(), String> {
        match self.users.get_mut(&id) {
            Some(state) => {
                let stocks_number = state
                    .stocks
                    .get_mut(&company.clone())
                    .ok_or("Not enough stocks".to_string())?;
                if *stocks_number == 0 {
                    return Err("Not enough stocks".to_string());
                }

                let Sold { price } = self.market_client.sell(company.clone()).await?;
                *stocks_number -= 1;
                state.balance += price;
                Ok(())
            }
            None => Err("user not found".to_string()),
        }
    }
}
