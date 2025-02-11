use std::path::Path;

use anyhow::{bail, Context};
use calamine::{Data, Range, Reader, Xlsx};

use crate::roster::EventType;


/// Get roster worksheet
pub fn get_roster_worksheet(path: impl AsRef<Path>) -> anyhow::Result<Range<Data>> {
    let mut workbook: Xlsx<_>  = calamine::open_workbook(path).context("Could not open spreadsheet")?;
    let worksheet = workbook.worksheet_range("Roster").context("Unable to locate 'Roster' worksheet")?;
    
    // Check worksheet is sufficiently large to avoid bounds checking in other functions
    let (num_rows, num_columns) = worksheet.get_size();
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

pub fn find_first_and_last_day_columns(worksheet: &Range<Data>, header_row: usize, name_column: usize, num_of_days_in_month: usize) -> anyhow::Result<(usize, usize)> {
    let mut rows = worksheet.rows();
    let header_row = rows.nth(header_row).unwrap();
    // Find first day
    let mut first = None;
    for (i, cell) in (&header_row[(name_column + 1)..(name_column + 6)]).iter().enumerate() {
        if let Data::Float(1.0) = cell {
            first = Some(name_column + 1 + i);
            break;
        }
    }
    let first = first.context("Unable to locate days row")?;

    // Find last day. All months have >= 28 days
    if let Some(last) = header_row.get(first + num_of_days_in_month - 1) {
        if let Data::Float(value) = last {
            if *value as usize == num_of_days_in_month {
                return Ok((first, first + num_of_days_in_month - 1));
            }
        }
    }
    
    bail!("Unable to locate last day of month in spreadsheet")
}

pub fn enumerate_names(worksheet: &Range<Data>, header_row: usize, name_column: usize) -> Vec<(usize, String)> {
    let mut names = Vec::new();
    let mut current_row = header_row;
    let last_row_to_check = 60 - header_row;
    let mut rows = worksheet.rows();
    rows.nth(header_row);
    while let Some(row) = rows.next() {
        current_row += 1;
        if current_row == last_row_to_check + 1 { break; }

        if let Data::String(value) = &row[name_column] {
            if value.is_empty() || value.starts_with("WATCH") {
                continue;
            }
            else if value.starts_with("OSS") {
                break;
            }
            else {
                names.push((current_row, value.clone()));
            }
        }

    }
    names
}

pub fn get_month_events_for_row(worksheet: &Range<Data>, row: usize, first_day_col: usize, last_day_col: usize) -> anyhow::Result<Vec<EventType>> {
    let mut vec = Vec::with_capacity(last_day_col - first_day_col + 1);
    let mut rows = worksheet.rows();
    let row = rows.nth(row).context(format!("Error parsing roster. Unable to locate row {row} in spreadsheet"))?;
    let days = row.get(first_day_col..=last_day_col).context("Not enough columns in row")?;
    for day in days {
        if let Data::String(value) = day {
            vec.push(EventType::from(value.as_str()));
        }
        else {
            bail!("Non-string data type in roster row");
        }
    }
    
    Ok(vec)
}