use std::path::Path;

use anyhow::{bail, Context};
use calamine::{Data, Range, Reader, Xlsx};


/// Get roster worksheet
pub fn get_roster_worksheet(path: impl AsRef<Path>) -> anyhow::Result<Range<Data>> {
    let mut workbook: Xlsx<_>  = calamine::open_workbook(path).context("Could not open spreadsheet")?;
    let worksheet = workbook.worksheet_range("Roster").context("Unable to locate 'Roster' worksheet")?;
    
    // Check worksheet is sufficiently large to avoid bounds checking in other functions
    let (num_rows, num_columns) = worksheet.get_size();
    println!("Rows: {}, columns: {}", num_rows, num_columns);
    if num_rows < 60 || num_columns < 33 {
        bail!("Worksheet is too small")
    }
    Ok(worksheet)
}

pub fn find_header_row_and_name_column(worksheet: &Range<Data>) -> anyhow::Result<(usize, usize)> {
    let rows = worksheet.rows();

    // Find header row
    for (i, row) in rows.enumerate() {
    // If we've got to 10, there's a problem
    if i == 10 {
        bail!("Header row not found in worksheet")
    }    
        
        // Find name column
        for j in 0..5 {
            match &row[j] {
                Data::String(s) if s == "NAME" => return Ok((i, j)),
                _ => continue,
            }
        }
    }


    bail!("Header row not found in worksheet")
}

pub fn find_first_and_last_day_columns(worksheet: &Range<Data>, header_row: usize, name_column: usize) -> anyhow::Result<(usize, usize)> {
    let mut rows = worksheet.rows();
    let header_row = rows.nth(header_row).unwrap();

    let mut first = 0;
    for cell in &header_row[(name_column + 1)..(name_column + 6)] {
        if let Data::Int(1) = cell {
            first 
        }
    }
    todo!()
}