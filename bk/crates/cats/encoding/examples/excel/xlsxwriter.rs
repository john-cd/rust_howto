// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Writes an `.xlsx` workbook with `rust_xlsxwriter`: a bold header,
//! numbers, dates, a formula and a column width.

use std::path::Path;

use rust_xlsxwriter::ExcelDateTime;
use rust_xlsxwriter::Format;
use rust_xlsxwriter::Formula;
use rust_xlsxwriter::Workbook;
use rust_xlsxwriter::XlsxError;

/// Writes a small sales report to `path`.
pub fn write_report(path: &Path) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Formats are values. Build them once and reuse them.
    let header = Format::new().set_bold();
    // Excel stores a date as a number. The number format makes it look like a
    // date.
    let date_format = Format::new().set_num_format("yyyy-mm-dd");

    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Sales")?;
    worksheet.set_column_width(0, 14)?;

    // Row and column indices are zero-based: (0, 0) is cell A1.
    let titles = ["Item", "Quantity", "Price", "Date", "Total"];
    for (col, title) in (0_u16..).zip(titles) {
        worksheet.write_with_format(0, col, title, &header)?;
    }

    worksheet.write(1, 0, "Widget")?;
    worksheet.write(1, 1, 3)?;
    worksheet.write(1, 2, 2.5)?;
    worksheet.write_with_format(
        1,
        3,
        &ExcelDateTime::from_ymd(2024, 1, 15)?,
        &date_format,
    )?;
    // A formula is stored as text. Excel recalculates it when the file opens.
    // The optional cached result lets readers that do not calculate, such as
    // `calamine`, see a value.
    worksheet.write_formula(1, 4, Formula::new("=B2*C2").set_result("7.5"))?;

    worksheet.write(2, 0, "Gadget")?;
    worksheet.write(2, 1, 10)?;
    worksheet.write(2, 2, 4)?;
    // The date of this row is left empty on purpose.
    worksheet.write_formula(2, 4, Formula::new("=B3*C3").set_result("40"))?;

    // A formula that Excel evaluates to an error. The cached result is an
    // error code, which readers see as an error cell.
    worksheet.write(3, 0, "Freebie")?;
    worksheet.write(3, 1, 5)?;
    worksheet.write(3, 2, 0)?;
    worksheet.write_formula(
        3,
        4,
        Formula::new("=B4/C4").set_result("#DIV/0!"),
    )?;

    workbook.save(path)?;
    Ok(())
}

/// Writes the report to a temporary directory.
fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("sales.xlsx");
    write_report(&path)?;

    // An `.xlsx` file is a ZIP archive: it starts with the bytes `PK`.
    let bytes = std::fs::read(&path)?;
    assert!(bytes.starts_with(b"PK"));
    println!("Wrote {} bytes to {}", bytes.len(), path.display());
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the xlsxwriter example through its entry point.
    #[test]
    fn test_xlsxwriter_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
