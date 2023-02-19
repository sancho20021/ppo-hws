use std::time::Duration;

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

pub struct SearchServerInfo {
    pub server: MockServer,
    pub request_url: url::Url,
    pub response_time: Duration,
}

pub fn start_server(returned_top: [&str; 5], response_time: Duration) -> SearchServerInfo {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let mock_server = MockServer::start().await;

            let get_route = "/get_top";

            let response = ResponseTemplate::new(200)
                .set_body_json(returned_top.clone())
                .set_delay(response_time);

            Mock::given(method("GET"))
                .and(path(get_route))
                .respond_with(response)
                .mount(&mock_server)
                .await;

            let request_url = {
                let mut request_url = url::Url::parse("http://localhost").unwrap();
                request_url
                    .set_port(Some(mock_server.address().port()))
                    .unwrap();
                request_url.set_path(get_route);
                request_url
            };

            SearchServerInfo {
                server: mock_server,
                request_url,
                response_time,
            }
        })
}
