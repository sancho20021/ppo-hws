use std::collections::HashMap;
use std::time::Duration;

use actix::prelude::*;

use crate::browser::Browser;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ResultsTop {
    pub api: String,
    pub top: Vec<String>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Query {
    pub query: String,
    pub master_actor: Addr<MasterActor>,
}

pub struct ChildActor {
    pub top_results_getter: Browser,
}

impl Actor for ChildActor {
    type Context = Context<Self>;
}

impl Handler<Query> for ChildActor {
    type Result = ();

    fn handle(&mut self, msg: Query, ctx: &mut Self::Context) -> Self::Result {
        let searcher = self.top_results_getter.clone();
        let fut = async move {
            let answer = searcher.search(&msg.query).await;
            match msg.master_actor.try_send(ResultsTop {
                api: searcher.api,
                top: answer,
            }) {
                Ok(_) => {}
                Err(e) => println!("Master actor already dead: {}", e),
            }
        };
        ctx.spawn(actix::fut::wrap_future(fut));
    }
}

pub struct MasterActor {
    pub apis: Vec<Addr<ChildActor>>,
    pub results: HashMap<String, Vec<String>>,
    pub results_storage: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Actor for MasterActor {
    type Context = Context<Self>;

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Master Actor is stopped");
        let mut results_storage = self.results_storage.try_lock().unwrap();
        *results_storage = self.results.clone();
    }
}

impl Handler<ResultsTop> for MasterActor {
    type Result = ();

    fn handle(&mut self, msg: ResultsTop, ctx: &mut Self::Context) {
        self.results.insert(msg.api, msg.top);
        if self.results.len() == self.apis.len() {
            ctx.stop();
            System::current().stop();
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AggregatorQuery {
    pub query: String,
    pub timeout: Duration,
}

impl Handler<AggregatorQuery> for MasterActor {
    type Result = ();

    fn handle(&mut self, msg: AggregatorQuery, ctx: &mut Self::Context) -> Self::Result {
        for api in self.apis.iter() {
            api.try_send(Query {
                query: msg.query.clone(),
                master_actor: ctx.address(),
            })
            .unwrap();
        }
        TimerActor
            .start()
            .try_send(StartTimer {
                duration: msg.timeout,
                master_actor: ctx.address(),
            })
            .unwrap();
    }
}

pub struct TimerActor;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartTimer {
    duration: Duration,
    master_actor: Addr<MasterActor>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct TimeOut;

impl Actor for TimerActor {
    type Context = Context<Self>;
}

impl Handler<StartTimer> for TimerActor {
    type Result = ();

    fn handle(&mut self, msg: StartTimer, ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            tokio::time::sleep(msg.duration).await;
            msg.master_actor.try_send(TimeOut).unwrap();
        };
        ctx.spawn(actix::fut::wrap_future(fut));
    }
}

impl Handler<TimeOut> for MasterActor {
    type Result = ();

    fn handle(&mut self, _msg: TimeOut, ctx: &mut Self::Context) -> Self::Result {
        println!("Time out");
        ctx.stop();
        System::current().stop();
    }
}
