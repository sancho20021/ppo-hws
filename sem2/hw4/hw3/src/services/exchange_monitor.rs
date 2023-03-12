use serde::{Deserialize, Serialize};

/// Mock Exchange Rate service
#[derive(Debug, Clone)]
pub struct ExchangeRateMonitor;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub to_usd: f64,
    pub to_rub: f64,
}

impl ExchangeRateMonitor {
    pub async fn get_exchange_rate(&self) -> ExchangeRate {
        ExchangeRate {
            to_usd: 1.05,
            to_rub: 80.44,
        }
    }
}
