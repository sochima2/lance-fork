use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::AppState, error::Result};

const DEFAULT_RECOVERY_LIMIT: i64 = 50;
const MAX_RECOVERY_LIMIT: i64 = 200;

pub fn router() -> Router<AppState> {
    Router::new().route("/write-recovery", get(list_write_recovery))
}

#[derive(Debug, Deserialize)]
struct RecoveryQuery {
    status: Option<String>,
    limit: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct WriteRecoveryRecord {
    id: Uuid,
    idempotency_key: String,
    operation: String,
    entity_type: String,
    entity_id: Option<Uuid>,
    status: String,
    attempts: i32,
    last_error: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[tracing::instrument(skip(state), fields(status = query.status.as_deref().unwrap_or("any")))]
async fn list_write_recovery(
    State(state): State<AppState>,
    Query(query): Query<RecoveryQuery>,
) -> Result<Json<Vec<WriteRecoveryRecord>>> {
    let limit = query
        .limit
        .unwrap_or(DEFAULT_RECOVERY_LIMIT)
        .clamp(1, MAX_RECOVERY_LIMIT);

    let rows = if let Some(status) = query.status {
        sqlx::query_as::<_, WriteRecoveryRecord>(
            r#"SELECT id, idempotency_key, operation, entity_type, entity_id, status,
                      attempts, last_error, created_at, updated_at
               FROM write_recovery_records
               WHERE status = $1
               ORDER BY updated_at DESC, id DESC
               LIMIT $2"#,
        )
        .bind(status)
        .bind(limit)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, WriteRecoveryRecord>(
            r#"SELECT id, idempotency_key, operation, entity_type, entity_id, status,
                      attempts, last_error, created_at, updated_at
               FROM write_recovery_records
               ORDER BY updated_at DESC, id DESC
               LIMIT $1"#,
        )
        .bind(limit)
        .fetch_all(&state.pool)
        .await?
    };

    tracing::debug!(records = rows.len());
    Ok(Json(rows))
}
