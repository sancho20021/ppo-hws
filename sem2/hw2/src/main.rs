use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use domain::{Card, CardCommand};
use postgres_es::{default_postgress_pool, PostgresCqrs};
use queries::SharedReportQuery;

mod config;
mod domain;
mod queries;

#[derive(Clone)]
struct AppState {
    cqrs: Arc<PostgresCqrs<Card>>,
    report_query: SharedReportQuery,
}

impl AppState {
    pub fn new(x: (PostgresCqrs<Card>, SharedReportQuery)) -> Self {
        Self {
            cqrs: Arc::new(x.0),
            report_query: x.1,
        }
    }
}

#[tokio::main]
async fn main() {
    let connection_string = "postgresql://pakulik@localhost:5432/pakulik";
    let pool = default_postgress_pool(connection_string).await;
    let shared_state = Arc::new(AppState::new(config::cqrs_framework(pool)));

    let router = Router::new()
        .route("/card/:card_id", post(command_handler))
        .route("/statistics", get(report_service_handler))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn report_service_handler(State(state): State<Arc<AppState>>) -> Response {
    let stats = state.report_query.get_statistics().await;
    (StatusCode::OK, Json(stats)).into_response()
}

async fn command_handler(
    Path(card_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(command): Json<CardCommand>,
) -> Response {
    match state.cqrs.execute(&card_id, command).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use cqrs_es::{mem_store::MemStore, CqrsFramework};

    use crate::{
        domain::{Card, CardCommand, UtcDateTime},
        queries::SharedReportQuery,
    };

    fn configure_cqrs() -> (CqrsFramework<Card, MemStore<Card>>, SharedReportQuery) {
        let event_store = MemStore::<Card>::default();
        let query = SharedReportQuery::new();
        (
            CqrsFramework::new(event_store, vec![Box::new(query.clone())], ()),
            query,
        )
    }

    fn before_n_days(n: i64) -> UtcDateTime {
        Utc::now() - Duration::days(n)
    }

    #[tokio::test]
    async fn test1() {
        let (cqrs, report_service) = configure_cqrs();
        let id = "1";
        cqrs.execute(
            id,
            CardCommand::IssueCard {
                issued: before_n_days(1),
                until: before_n_days(-1),
            },
        )
        .await
        .unwrap();
        cqrs.execute(id, CardCommand::Enter { time: Utc::now() })
            .await
            .unwrap();
        println!("query: {:?}", report_service.get_statistics().await);
    }
}
