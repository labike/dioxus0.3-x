use diesel::{ConnectionError, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{error::Error, sync::Arc};

use crate::error::QueryError;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub type AsyncConnection = PgConnection;
pub type OwnedAsyncConnection = PgConnection;

#[derive(Clone, Debug, Default)]
pub struct PoolState {
    pub max_size: usize,
}

#[derive(Clone, Debug)]
pub struct AsyncConnectionPool {
    connection_url: Arc<str>,
}

impl AsyncConnectionPool {
    pub async fn new<S: AsRef<str>>(url: S) -> Result<Self, QueryError> {
        let pool = new_async_pool(url).await?;
        {
            // check connection
            let _ = pool.get_owned().await?;
        }
        Ok(pool)
    }

    pub async fn get(&self) -> Result<AsyncConnection, QueryError> {
        self.get_owned().await
    }

    pub async fn get_owned(&self) -> Result<OwnedAsyncConnection, QueryError> {
        connect(self.connection_url.as_ref()).map_err(|e| QueryError::Connection(e.to_string()))
    }

    pub fn state(&self) -> PoolState {
        PoolState { max_size: 1 }
    }
}

/// Run database migrations
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

/// Connect to the database
pub fn connect<S: AsRef<str>>(url: S) -> Result<PgConnection, ConnectionError> {
    use diesel::Connection;
    let url = url.as_ref();
    PgConnection::establish(url)
}

/// Usage:
/// ```ignore
/// let async_pool = new_async_pool("postgres://login@localhost/sample").await;
/// let conn = &mut async_pool.get().await?;
/// ```
pub async fn new_async_pool<S: AsRef<str>>(url: S) -> Result<AsyncConnectionPool, QueryError> {
    Ok(AsyncConnectionPool {
        connection_url: Arc::<str>::from(url.as_ref()),
    })
}
