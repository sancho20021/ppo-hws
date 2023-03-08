use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use chrono::{Datelike, Utc};
use cqrs_es::{
    persist::{GenericQuery, PersistenceError, ViewContext, ViewRepository},
    Aggregate, Query, View,
};
use serde::{Deserialize, Serialize};

use crate::domain::{Card, CardEvent, CardId, UtcDateTime};

type MyViewRepository = ViewInMemoryRepository<ReportServiceView, Card>;
type MyQuery = GenericQuery<MyViewRepository, ReportServiceView, Card>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportServiceView {
    /// today is [0]
    pub last_week_visits: [usize; 7],
    pub last_measurement: UtcDateTime,
    pub total_visits: u64,
    pub view_created: UtcDateTime,
    pub average_duration_hours: f64,
    pub visiting_cards: HashMap<CardId, UtcDateTime>,
}

impl Default for ReportServiceView {
    fn default() -> Self {
        Self {
            last_week_visits: Default::default(),
            last_measurement: Default::default(),
            total_visits: 0,
            view_created: Utc::now(),
            average_duration_hours: Default::default(),
            visiting_cards: Default::default(),
        }
    }
}

impl View<Card> for ReportServiceView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Card>) {
        let now = Utc::now();
        let last_measurement_day = self.last_measurement.day();
        let now_day = now.day();
        let days_skipped = (now_day - last_measurement_day) as usize;
        self.last_week_visits.rotate_right(days_skipped);
        self.last_week_visits[0..(std::cmp::min(7, days_skipped))].fill(0);

        let id = &event.aggregate_id;
        match event.payload {
            CardEvent::CardEntered { time } => {
                self.visiting_cards.insert(id.clone(), time);
                self.last_week_visits[0] += 1;
                self.total_visits += 1;
            }
            CardEvent::CardLeft { time } => {
                let person_entered = self.visiting_cards.remove(id).unwrap();
                let another_duration =
                    ((time - person_entered).num_seconds() as f64) / (60.0 * 60.0);
                self.average_duration_hours +=
                    (another_duration - self.average_duration_hours) / (self.total_visits as f64);
            }
            _ => {}
        }
        self.last_measurement = now;
    }
}

pub struct ViewInMemoryRepository<V, A>
where
    V: View<A> + Clone,
    A: Aggregate,
{
    view: Arc<Mutex<V>>,
    view_context: Arc<Mutex<MyViewContext>>,
    phantom_data: PhantomData<A>,
}

impl<V, A> ViewInMemoryRepository<V, A>
where
    V: View<A> + Clone,
    A: Aggregate,
{
    pub fn new() -> Self {
        ViewInMemoryRepository {
            view: Arc::new(Mutex::new(V::default())),
            view_context: Arc::new(Mutex::new(MyViewContext {
                view_instance_id: "sjsjsjsjs".to_string(),
                version: 0,
            })),
            phantom_data: PhantomData::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct MyViewContext {
    pub view_instance_id: String,
    pub version: i64,
}

impl From<ViewContext> for MyViewContext {
    fn from(value: ViewContext) -> Self {
        MyViewContext {
            view_instance_id: value.view_instance_id,
            version: value.version,
        }
    }
}

impl Into<ViewContext> for MyViewContext {
    fn into(self) -> ViewContext {
        ViewContext {
            view_instance_id: self.view_instance_id,
            version: self.version,
        }
    }
}

#[async_trait]
impl<V, A> ViewRepository<V, A> for ViewInMemoryRepository<V, A>
where
    V: View<A> + Clone,
    A: Aggregate,
{
    async fn load(&self, _view_id: &str) -> Result<Option<V>, PersistenceError> {
        return Ok(Some(self.view.lock().unwrap().clone()));
    }

    async fn load_with_context(
        &self,
        _view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, PersistenceError> {
        return Ok(Some((
            self.view.lock().unwrap().clone(),
            self.view_context.lock().unwrap().clone().into(),
        )));
    }

    async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        *self.view.lock().unwrap() = view;
        *self.view_context.lock().unwrap() = context.into();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitStatistics {
    measurement_date: UtcDateTime,
    visits: [usize; 7],
    average_daily_visits: f64,
    average_duration_hours: f64,
}

pub struct SharedReportQuery {
    query: Arc<MyQuery>,
}

impl SharedReportQuery {
    pub fn new() -> Self {
        Self {
            query: Arc::new(MyQuery::new(Arc::new(MyViewRepository::new()))),
        }
    }

    pub async fn get_statistics(&self) -> VisitStatistics {
        let x = self.query.load("view_id").await.unwrap();
        let measurement_date = x.last_measurement;
        let visits = x.last_week_visits;
        let average_daily_visits = x.total_visits as f64
            / std::cmp::max((measurement_date - x.view_created).num_days(), 1) as f64;
        let average_duration_hours = x.average_duration_hours;
        VisitStatistics {
            measurement_date,
            visits,
            average_daily_visits,
            average_duration_hours,
        }
    }
}

impl Clone for SharedReportQuery {
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
        }
    }
}

impl Query<Card> for SharedReportQuery {
    fn dispatch<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        aggregate_id: &'life1 str,
        events: &'life2 [cqrs_es::EventEnvelope<Card>],
    ) -> core::pin::Pin<
        Box<dyn core::future::Future<Output = ()> + core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        self.query.dispatch(aggregate_id, events)
    }
}
