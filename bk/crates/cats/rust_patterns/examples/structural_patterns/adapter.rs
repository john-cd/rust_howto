// ANCHOR: example
//! Adapts buffered input lines into simple comma-separated row values.

use std::io::BufRead;
use std::io::Cursor;
use std::io::Lines;

/// An iterator adapter that turns each input line into a row of fields.
struct CsvRows<R> {
    lines: Lines<R>,
}

impl<R: BufRead> CsvRows<R> {
    /// Creates a row iterator over a buffered reader.
    fn new(reader: R) -> Self {
        Self {
            lines: reader.lines(),
        }
    }
}

impl<R: BufRead> Iterator for CsvRows<R> {
    type Item = std::io::Result<Vec<String>>;

    /// Reads one line and splits it into fields.
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            line.map(|line| line.split(',').map(str::to_owned).collect())
        })
    }
}

/// Prints rows using the adapter over an in-memory reader.
fn main() -> anyhow::Result<()> {
    let input = Cursor::new("name,role\nAda,engineer\n");

    for row in CsvRows::new(input) {
        println!("{:?}", row?);
    }

    Ok(())
}
// ANCHOR_END: example

/// Runs the reader adapter example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::CsvRows;
    use super::main;

    /// Verifies the adapter yields each input line as a row.
    /// Runs the adapter example through its entry point.
    #[test]
    fn buffered_lines_become_rows() {
        let input = Cursor::new("name,role\nAda,engineer\n");
        let rows = CsvRows::new(input)
            .collect::<std::io::Result<Vec<_>>>()
            .expect("the in-memory input should be readable");

        assert_eq!(rows, vec![vec!["name", "role"], vec!["Ada", "engineer"]]);
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
