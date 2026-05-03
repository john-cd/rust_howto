#[cfg(feature = "sqlparser")]
mod sqlparser;

fn main() {
    #[cfg(feature = "sqlparser")]
    {
        sqlparser::run();
    }
}
