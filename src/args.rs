use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Args {
    /// The .xlsx file that contain's the monthly roster
    pub xlsx_path: PathBuf,

    /// Override the calendar month. If this is not specified, we will attempt to extract the month from the file name
    #[clap(short, value_enum)]
    pub month: Option<Month>,

    /// Override the year. If this is not specified, we will attempt to extract the year from the file name
    #[clap(short)]
    pub year: Option<u16>,

    /// Override the calendar name. If not specified, it will default to "My Calendar"
    #[clap(short)]
    pub name: Option<String>,
    
    /// Provide an e-mail address if you wish to receive an e-mail reminder of the event 60m beforehand. Override with -r flag to set different time
    #[clap(short)]
    pub email: Option<String>,

    /// How long before the event you wish to be notified / e-mailed. A number followed by m, h or d for minutes, hours or days
    #[clap(short)]
    pub reminder_time: Option<String>,

    /// The path to write the output .ics file to
    #[arg(short)]
    pub output_ics: PathBuf
}


#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Month {
    Jan = 1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec
}