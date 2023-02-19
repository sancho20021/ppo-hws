use url::Url;

#[derive(Debug, Clone)]
pub struct Browser {
    pub api: String,
    pub server_request_url: Url,
}

impl Browser {
    pub async fn search(&self, query: &String) -> Vec<String> {
        let mut url = self.server_request_url.clone();
        url.set_query(Some(format!("query={}", query).as_str()));

        reqwest::get(url)
            .await
            .unwrap()
            .json::<Vec<String>>()
            .await
            .unwrap()
    }
}
