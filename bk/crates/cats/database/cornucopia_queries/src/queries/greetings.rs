//! \ Generated\ query\ bindings\ for\ the\ greetings\ database\ schema\.
//!
//! TODO Reviewed ??-??-2026.

use futures::StreamExt;
use futures::TryStreamExt;
use futures::{self};

use crate::client::async_::GenericClient;

/// Reusable query description for reading greeting strings from the database.
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    /// Database client used to execute the query.
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],

    /// Query text sent to the database.
    query: &'static str,

    /// Optional cached used by string query; `None` means the value is not configured or
    /// unavailable.
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,

    /// Function that converts a database row to the result type.
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    /// Changes the function applied to each greeting row.
    ///
    /// # Arguments
    ///
    /// * `mapper` - Conversion applied to each extracted greeting string.
    ///
    /// # Returns
    ///
    /// A query with the same SQL and a new result type.
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }

    /// Executes the query and converts its single greeting row.
    ///
    /// # Errors
    ///
    /// Returns the database error or a row-extraction error.
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }

    /// Executes the query and converts all matching greeting rows.
    ///
    /// # Errors
    ///
    /// Returns the database or row-extraction error if iteration fails.
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }

    /// Executes the query and converts an optional greeting row.
    ///
    /// # Errors
    ///
    /// Returns the database error or a row-extraction error.
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }

    /// Executes the greeting query and returns its rows as a fallible stream.
    ///
    /// # Errors
    ///
    /// Returns an error if opening the query stream fails. Items in the returned stream can also
    /// report database or row-extraction errors.
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}

/// Prepared statement that inserts a greeting into the database.
pub struct AddGreetingStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn add_greeting() -> AddGreetingStmt {
    AddGreetingStmt(
        "INSERT INTO cornucopia_greetings (message) VALUES ($1) ON CONFLICT (message) DO NOTHING",
        None,
    )
}
impl AddGreetingStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }

    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        message: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[message]).await
    }
}

/// Groups the data required by queries operations.
pub struct ListGreetingsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_greetings() -> ListGreetingsStmt {
    ListGreetingsStmt(
        "SELECT message FROM cornucopia_greetings ORDER BY message",
        None,
    )
}
impl ListGreetingsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }

    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> StringQuery<'c, 'a, 's, C, String, 0> {
        StringQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
