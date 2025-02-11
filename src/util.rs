use std::io::Write;

use anyhow::Context;
use chrono::{NaiveDate, Datelike};

use crate::args::Args;


pub fn get_first_day_of_month(args: &Args) -> anyhow::Result<NaiveDate> {

    let file_name = args.xlsx_path.file_name().and_then(|os_str| os_str.to_str()).context("Invalid character in input file name")?;

    let mut year = args.year.map(|year| year as i32);
    if year.is_none() {
        for i in 2000..2100 {
            let year_string = i.to_string();
            if file_name.contains(&year_string) {
                year = Some(i);
                break;
            }
        }
    }
    let year = year.context("Unable to determine year from file name - try specifying it manually with the -y argument")?;

    let mut month = args.month.map(|month| month as u32);
    if month.is_none() {
        for (i, m) in ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"].iter().enumerate() {
            let i = i as u32 + 1;
            if file_name.contains(m) {
                month = Some(i);
                break;
            }
        }
    }
    let month = month.context("Unable to determine month from file name - try specifying it manually with the -m argument")?;

    let date = NaiveDate::from_ymd_opt(year, month, 1).context("Unable to determine month / year")?;
    Ok(date)
}

pub fn num_days_in_month(date: &NaiveDate) -> usize {
    let (_, year) = date.year_ce();
    let last_day_of_month = NaiveDate::from_ymd_opt(year as i32, date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1).unwrap())
        .pred_opt().unwrap();
    last_day_of_month.day() as usize
}

pub fn get_time_from_user(prompt: &str) -> (u32, u32) {
    loop {
        println!();
        println!("{prompt}");
        print!("Enter the time as four digits in 24hr format, e.g. 0830: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input_tr = input.trim();
        if input_tr.len() != 4 {
            println!("Invalid input.");
            continue;
        }
        let as_num = match input_tr.parse::<u32>() {
            Ok(as_num) => as_num,
            _ => {
                println!("Invalid input.");
                continue;
            },
        };
        
        let (hours, mins) = (as_num / 100, as_num % 100);
        if hours > 23 || mins > 59 {
            println!("Invalid input.");
                continue;
        }
        break (hours, mins);
    }
}