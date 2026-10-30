<!-- TODO Reviewed ??-??-2026 -->

[`tiberius`][c~tiberius~docs] implements the SQL Server TDS protocol for
asynchronous Rust applications. This example pairs it with Tokio and native
TLS, verifies the server certificate by default, and uses SQL logins. SQLx
removed its experimental MSSQL driver in 0.7; a current SQLx release does not
connect to SQL Server. Integrated authentication has separate platform,
feature, and domain requirements.

| Recipe                             | Crates              | Categories                                     |
| ---------------------------------- | ------------------- | ---------------------------------------------- |
| [`tiberius`][ex~database~tiberius] | {{#crate tiberius}} | {{#categories database="Database interfaces"}} |
