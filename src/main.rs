use std::io::{self, Write};

use anyhow::bail;
use clap::Parser;
use roster::EventType;

mod args;
mod xlsx;
mod util;
mod roster;
mod ics;

fn main() -> Result<(), anyhow::Error> {
    let args = args::Args::parse();
    if let Some(year) = args.year {
        if !(2000..2100).contains(&year) {
            bail!("Invalid year ({year}). Must be between 2000 and 2099.");
        }
    }

    let first_day_of_month = util::get_first_day_of_month(&args)?;
    let num_of_days_in_month = util::num_days_in_month(&first_day_of_month);
    let worksheet = xlsx::get_roster_worksheet(&args.xlsx_path)?;
    let (header_row, name_column) = xlsx::find_header_row_and_name_column(&worksheet)?;
    let (first, last) = xlsx::find_first_and_last_day_columns(&worksheet, header_row, name_column, num_of_days_in_month)?;
    let names = xlsx::enumerate_names(&worksheet, header_row, name_column);
    
    // Prompt to select name
    for (i, (_, name)) in names.iter().enumerate() {
        println!("{:>2}: {}", i + 1, name);
    }
    let selected_name_index = loop {
        print!("For whom do you wish to generate a calendar? Enter a number from 1 - {}: ", names.len());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(i) if (1..=names.len()).contains(&i) => break i - 1,
            _ => {
                println!("Invalid input.");
                continue;
            }
        };
    };

    let mut days = xlsx::get_month_events_for_row(&worksheet, names[selected_name_index].0, first, last)?;
    
    let mut current_day = first_day_of_month.clone();
    for day in days.iter_mut() {
        if let EventType::Other { name, .. } = day {
            let date = current_day.format("%d %B");
            
            let (hour_start, minute_start, hour_end, minute_end) = loop {
                let (hour_start, minute_start) = util::get_time_from_user(&format!("When does event '{name}' on {date} start?"));
                let (hour_end, minute_end) = util::get_time_from_user(&format!("When does event '{name}' on {date} end?"));
                if hour_start * 100 + minute_start >= hour_end * 100 + minute_end {
                    println!("End time must be after the start time.");
                    continue;
                }
                break (hour_start, minute_start, hour_end, minute_end);
            };

            *day = EventType::Other { name: name.clone(), hour_start, minute_start, hour_end, minute_end };
            
        }
        current_day = current_day.succ_opt().unwrap();
    }

    // Convert to list of events
    let event_list = roster::generate_calendar_events(first_day_of_month, days);
    println!("{:#?}", event_list);

    Ok(())
}
