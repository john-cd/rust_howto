#[cfg(feature = "mssql")]
mod tiberius;

fn main() {
    #[cfg(feature = "mssql")]
    {
        tiberius::run();
    }
}
