use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use actix::prelude::*;

use crate::{
    actors::{AggregatorQuery, ChildActor, MasterActor},
    browser::Browser,
    server::start_server,
};

mod actors;
mod browser;
mod server;

#[derive(Message)]
#[rtype(result = "String")]
struct Message(&'static str);

fn browse(
    query: String,
    browsers: Vec<Browser>,
    timeout: Duration,
) -> HashMap<String, Vec<String>> {
    let results_storage = Arc::new(Mutex::new(HashMap::new()));
    let sys = System::new();
    let now = SystemTime::now();

    sys.block_on(async {
        let child_actors: Vec<Addr<ChildActor>> = browsers
            .into_iter()
            .map(|top_results_getter| ChildActor { top_results_getter }.start())
            .into_iter()
            .collect();

        let master_actor = MasterActor {
            apis: child_actors,
            results: Default::default(),
            results_storage: results_storage.clone(),
        }
        .start();
        println!("Starting aggregating query");
        master_actor
            .try_send(AggregatorQuery { query, timeout })
            .unwrap();
    });

    sys.run().unwrap();

    println!("Time taken: {:?}", now.elapsed().unwrap());

    let s = results_storage.try_lock().unwrap();
    s.clone()
}

fn main() {
    let yandex_server = start_server(["Ya", "Ya", "a", "n", "d"], Duration::ZERO);
    let google_server = start_server(["Go", "Go", "g", "l", "e"], Duration::ZERO);
    let browser1 = Browser {
        api: "Yandex".to_string(),
        server_request_url: yandex_server.request_url,
    };
    let browser2 = Browser {
        api: "Google".to_string(),
        server_request_url: google_server.request_url,
    };
    let results = browse(
        "kizaru".to_string(),
        vec![browser1, browser2],
        Duration::from_secs(3),
    );
    println!("Results: {:?}", results);
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use crate::{browse, browser::Browser, server::start_server};

    fn to_string_vec(top: [&str; 5]) -> Vec<String> {
        top.into_iter().map(|s| s.to_string()).collect()
    }

    fn to_hashmap(api_tops: &[(&str, [&str; 5])]) -> HashMap<String, Vec<String>> {
        api_tops
            .into_iter()
            .map(|(s, t)| (s.to_string(), to_string_vec(*t)))
            .collect()
    }

    #[test]
    fn test_full_results() {
        let yandex_top = ["Ya", "Ya", "a", "n", "d"];
        let yandex_server = start_server(yandex_top, Duration::ZERO);
        let google_top = ["Go", "Go", "g", "l", "e"];
        let google_server = start_server(google_top, Duration::ZERO);

        let browser1 = Browser {
            api: "Yandex".to_string(),
            server_request_url: yandex_server.request_url,
        };
        let browser2 = Browser {
            api: "Google".to_string(),
            server_request_url: google_server.request_url,
        };
        let results = browse(
            "kizaru".to_string(),
            vec![browser1, browser2],
            Duration::from_secs(3),
        );
        assert_eq!(
            results,
            to_hashmap(&[("Yandex", yandex_top), ("Google", google_top)])
        );
    }

    #[test]
    fn test_half_results() {
        let yandex_top = ["Ya", "Ya", "a", "n", "d"];
        let yandex_server = start_server(yandex_top, Duration::ZERO);
        let google_top = ["Go", "Go", "g", "l", "e"];
        let google_server = start_server(google_top, Duration::from_millis(500));

        let browser1 = Browser {
            api: "Yandex".to_string(),
            server_request_url: yandex_server.request_url,
        };
        let browser2 = Browser {
            api: "Google".to_string(),
            server_request_url: google_server.request_url,
        };
        let results = browse(
            "kizaru".to_string(),
            vec![browser1, browser2],
            Duration::from_millis(100),
        );
        assert_eq!(results, to_hashmap(&[("Yandex", yandex_top)]));
    }

    #[test]
    fn test_empty_results() {
        let yandex_top = ["Ya", "Ya", "a", "n", "d"];
        let yandex_server = start_server(yandex_top, Duration::from_millis(500));
        let google_top = ["Go", "Go", "g", "l", "e"];
        let google_server = start_server(google_top, Duration::from_millis(500));

        let browser1 = Browser {
            api: "Yandex".to_string(),
            server_request_url: yandex_server.request_url,
        };
        let browser2 = Browser {
            api: "Google".to_string(),
            server_request_url: google_server.request_url,
        };
        let results = browse(
            "kizaru".to_string(),
            vec![browser1, browser2],
            Duration::from_millis(100),
        );
        assert_eq!(results, Default::default());
    }
}
