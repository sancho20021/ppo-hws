use std::time::{Duration, SystemTime};

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "String")]
struct Message(&'static str);

type FutureResultsTop = ResponseFuture<Result<ResultsTop, String>>;

struct TopResultsAggregator {
    apis: Vec<Addr<TopResultsGetter>>,
}

impl Actor for TopResultsAggregator {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<ResultsTop, String>")]
struct AggregatorQuery(String);

impl Handler<AggregatorQuery> for TopResultsAggregator {
    type Result = FutureResultsTop;

    fn handle(&mut self, msg: AggregatorQuery, ctx: &mut Self::Context) -> Self::Result {
        let api_results: Vec<_> = self
            .apis
            .iter()
            .map(|api| api.send(Query(msg.0.clone())))
            .collect();

        System::current().stop();

        Box::pin(async move {
            let results = futures::future::join_all(api_results).await;
            let x: Result<Vec<Vec<String>>, _> = results
                .into_iter()
                .map(|r| match r {
                    Ok(r) => r.map(|x| x.0),
                    Err(e) => Err(e.to_string())?,
                })
                .collect();
            Ok(ResultsTop(x?.concat()))
        })
    }
}

struct ResultsTop(Vec<String>);

#[derive(Message)]
#[rtype(result = "(Result<ResultsTop, String>)")]
struct Query(String);

struct TopResultsGetter {
    api: String,
}

impl Actor for TopResultsGetter {
    type Context = Context<Self>;
}

impl Handler<Query> for TopResultsGetter {
    type Result = ResponseFuture<Result<ResultsTop, String>>;

    fn handle(&mut self, msg: Query, _ctx: &mut Self::Context) -> Self::Result {
        println!("API {} is querying", self.api);
        let duration = if msg.0.len() > 3 {
            Duration::from_secs(3)
        } else {
            Duration::from_millis(100)
        };
        println!("query {} will take {:?}", msg.0, duration);
        Box::pin(async move {
            tokio::time::sleep(duration).await;
            Ok(ResultsTop(vec!["hello".to_string(); 5]))
        })
    }
}

fn main() {
    let sys = System::new();
    let now = SystemTime::now();
    sys.block_on(async {
        let apis: Vec<Addr<TopResultsGetter>> = ["Yandex", "Google"]
            .map(|api| {
                TopResultsGetter {
                    api: api.to_string(),
                }
                .start()
            })
            .into_iter()
            .collect();
        let aggregator = TopResultsAggregator { apis }.start();

        println!("Starting aggregating query");
        let answer = aggregator
            .send(AggregatorQuery("kizaru".to_string()))
            .await
            .unwrap()
            .unwrap();
        println!("Answer: {:?}", answer.0);
    });

    sys.run().unwrap();

    println!("Time taken: {:?}", now.elapsed().unwrap());
}
