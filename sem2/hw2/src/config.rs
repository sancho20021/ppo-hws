use cqrs_es::{persist::PersistedEventStore, CqrsFramework, Query};
use postgres_es::{PostgresEventRepository};
use sqlx::{Pool, Postgres};

use crate::{domain::Card, queries::SharedReportQuery};

pub fn cqrs_framework(
    pool: Pool<Postgres>,
) -> (
    CqrsFramework<Card, PersistedEventStore<PostgresEventRepository, Card>>,
    SharedReportQuery,
) {
    let query = SharedReportQuery::new();
    let query_in_box: Box<dyn Query<Card>> = Box::new(query.clone());
    let queries = vec![query_in_box];
    (postgres_es::postgres_cqrs(pool, queries, ()), query)
}
