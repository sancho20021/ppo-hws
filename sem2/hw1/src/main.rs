use std::time::{Duration, SystemTime};

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

// type FutureResultsTop = ResponseFuture<Result<ResultsTop, String>>;

fn main() {
    let sys = System::new();
    let now = SystemTime::now();
    sys.block_on(async {
        let child_actors: Vec<Addr<ChildActor>> = ["Yandex", "Google"]
            .map(|api| {
                ChildActor {
                    top_results_getter: TopResultsGetter {
                        api: api.to_string(),
                    },
                }
                .start()
            })
            .into_iter()
            .collect();

        let master_actor = MasterActor {
            apis: child_actors,
            results: Default::default(),
        }
        .start();
        println!("Starting aggregating query");
        master_actor
            .try_send(AggregatorQuery {
                query: "kizaru".to_string(),
                timeout: Duration::from_secs(3),
            })
            .unwrap();
    });

    sys.run().unwrap();

    println!("Time taken: {:?}", now.elapsed().unwrap());
}
