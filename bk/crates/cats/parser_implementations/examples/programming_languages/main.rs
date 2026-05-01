#[cfg(feature = "sqlparser")]
mod sqlparser;

fn main() {
    #[cfg(feature = "sqlparser")]
    {
        let _ = sqlparser::run();
    }
}
