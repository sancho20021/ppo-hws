use std::time::Duration;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct TopResultsGetter {
    pub api: String,
}

impl TopResultsGetter {
    pub async fn search(&self, query: &String) -> Vec<String> {
        println!("API {} is querying", self.api);
        let duration = Duration::from_secs(rand::thread_rng().gen_range(1..5));
        println!("query {} will take {:?}", query, duration);
        tokio::time::sleep(duration).await;
        vec!["hello".to_string(); 5]
    }
}
