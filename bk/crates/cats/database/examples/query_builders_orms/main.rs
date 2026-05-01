#[cfg(feature = "diesel")]
mod diesel1;
#[cfg(feature = "sea_orm")]
mod sea_orm;
#[cfg(feature = "sqlx")]
mod sqlx;

fn main() {}
