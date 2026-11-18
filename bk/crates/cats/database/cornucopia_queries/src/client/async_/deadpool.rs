//! \ Deadpool\ connection-pool\ integration\ for\ the\ generated\ async\ database\ client\.
//!
//! TODO Reviewed ??-??-2026.

use deadpool_postgres::Client as DeadpoolClient;
use deadpool_postgres::ClientWrapper;
use deadpool_postgres::Transaction as DeadpoolTransaction;
use tokio_postgres::Client as PgClient;
use tokio_postgres::Error;
use tokio_postgres::RowStream;
use tokio_postgres::Statement;
use tokio_postgres::ToStatement;
use tokio_postgres::Transaction as PgTransaction;
use tokio_postgres::types::BorrowToSql;

use super::generic_client::GenericClient;
impl GenericClient for DeadpoolClient {
    /// Enables helper-level preparation through the pooled client's statement cache.
    fn stmt_cache() -> bool {
        true
    }

    /// Prepares the request or statement so it can be executed safely.
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        ClientWrapper::prepare_cached(self, query).await
    }

    /// Executes the prepared operation and reports its result.
    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::execute(self, query, params).await
    }

    /// Executes the query and requires exactly one returned row.
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_one(self, statement, params).await
    }

    /// Executes the query and returns an optional row.
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query_opt(self, statement, params).await
    }

    /// Executes the query with the supplied parameters.
    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgClient::query(self, query, params).await
    }

    /// Executes a raw database query and returns its untyped result.
    async fn query_raw<T, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        I: IntoIterator + Sync + Send,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        PgClient::query_raw(self, statement, params).await
    }
}
impl GenericClient for DeadpoolTransaction<'_> {
    /// Leaves automatic helper-level preparation disabled for this transaction adapter.
    fn stmt_cache() -> bool {
        false
    }

    /// Prepares the request or statement so it can be executed safely.
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        DeadpoolTransaction::prepare_cached(self, query).await
    }

    /// Executes the prepared operation and reports its result.
    async fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::execute(self, query, params).await
    }

    /// Executes the query and requires exactly one returned row.
    async fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<tokio_postgres::Row, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_one(self, statement, params).await
    }

    /// Executes the query and returns an optional row.
    async fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query_opt(self, statement, params).await
    }

    /// Executes the query with the supplied parameters.
    async fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, Error>
    where
        T: ?Sized + tokio_postgres::ToStatement + Sync + Send,
    {
        PgTransaction::query(self, query, params).await
    }

    /// Executes a raw database query and returns its untyped result.
    async fn query_raw<T, I>(&self, statement: &T, params: I) -> Result<RowStream, Error>
    where
        T: ?Sized + ToStatement + Sync + Send,
        I: IntoIterator + Sync + Send,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        PgTransaction::query_raw(self, statement, params).await
    }
}
