#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Reads an `.xlsx` workbook with `calamine`.
//!
//! The workbook is the one written by the `rust_xlsxwriter` recipe,
//! so the example needs no binary fixture.

use calamine::CellErrorType;
use calamine::Data;
use calamine::Reader;
use calamine::Xlsx;
use calamine::open_workbook;

use crate::xlsxwriter::write_report;

/// Reads the report and checks the type and value of each cell.
fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("sales.xlsx");
    write_report(&path)?;

    let mut workbook: Xlsx<_> = open_workbook(&path)?;
    assert_eq!(workbook.sheet_names(), ["Sales"]);

    // `worksheet_range` returns the used area of the sheet, with typed cells.
    let range = workbook.worksheet_range("Sales")?;
    assert_eq!(range.get_size(), (4, 5));

    for row in range.rows().skip(1) {
        println!("{row:?}");
    }

    // Cells are indexed from the top-left corner of the used area, starting at
    // 0.
    assert_eq!(range.get((0, 0)), Some(&Data::String("Item".to_owned())));
    assert_eq!(range.get((1, 0)), Some(&Data::String("Widget".to_owned())));
    // Numbers are stored as floating point in an `.xlsx` file, even whole ones.
    assert_eq!(range.get((1, 1)), Some(&Data::Float(3.0)));
    assert_eq!(range.get((1, 2)), Some(&Data::Float(2.5)));
    // Dates are serial numbers: days since 1899-12-30.
    // 2024-01-15 is day 45306.
    match range.get((1, 3)) {
        Some(Data::DateTime(date)) => assert_eq!(date.as_f64(), 45306.0),
        other => anyhow::bail!("expected a date, found {other:?}"),
    }
    // A cell that was never written is empty.
    assert_eq!(range.get((2, 3)), Some(&Data::Empty));
    // A formula cell holds its cached result...
    assert_eq!(range.get((1, 4)), Some(&Data::Float(7.5)));
    assert_eq!(range.get((2, 4)), Some(&Data::Float(40.0)));

    // A formula that fails is an error cell. Match on `Data::Error` instead of
    // treating the cell as a number.
    assert_eq!(range.get((3, 4)), Some(&Data::Error(CellErrorType::Div0)));

    // ...and the formula text, without the leading `=`, is available
    // separately. `get_value` takes sheet coordinates, unlike `get`, which
    // is relative to the used area.
    let formulas = workbook.worksheet_formula("Sales")?;
    assert_eq!(
        formulas.get_value((1, 4)).map(String::as_str),
        Some("B2*C2")
    );

    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
