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