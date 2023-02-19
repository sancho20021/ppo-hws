use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use actix::prelude::*;

use crate::{
    actors::{AggregatorQuery, ChildActor, MasterActor},
    results_getter::TopResultsGetter,
};

mod actors;
mod results_getter;

#[derive(Message)]
#[rtype(result = "String")]
struct Message(&'static str);

fn browse(
    query: String,
    browsers: Vec<TopResultsGetter>,
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
    let browsers: Vec<TopResultsGetter> = ["Yandex", "Google"]
        .map(|api| TopResultsGetter {
            api: api.to_string(),
        })
        .into_iter()
        .collect();

    let results = browse("kizaru".to_string(), browsers, Duration::from_secs(3));
    println!("Results: {:?}", results);
}
