use crate::services::judge::JudgeService;
use crate::services::stellar::StellarService;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DbPoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl DbPoolConfig {
    pub fn from_env() -> Self {
        Self {
            max_connections: env_u32("DB_MAX_CONNECTIONS", 10),
            min_connections: env_u32("DB_MIN_CONNECTIONS", 1),
            acquire_timeout_secs: env_u64("DB_ACQUIRE_TIMEOUT_SECS", 5),
            idle_timeout_secs: env_u64("DB_IDLE_TIMEOUT_SECS", 300),
            max_lifetime_secs: env_u64("DB_MAX_LIFETIME_SECS", 1800),
        }
    }
}

pub async fn connect_pool(database_url: &str, config: DbPoolConfig) -> sqlx::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(config.max_lifetime_secs))
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query(
                    "SET SESSION CHARACTERISTICS AS TRANSACTION ISOLATION LEVEL READ COMMITTED",
                )
                .execute(conn)
                .await?;
                Ok(())
            })
        })
        .connect(database_url)
        .await
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub judge: std::sync::Arc<JudgeService>,
    pub stellar: std::sync::Arc<StellarService>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            judge: std::sync::Arc::new(JudgeService::from_env()),
            stellar: std::sync::Arc::new(StellarService::from_env()),
        }
    }
}

fn env_u32(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
