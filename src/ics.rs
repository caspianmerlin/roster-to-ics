use chrono::NaiveDate;
use ics::{components::Property, properties::{CalScale, Method, RRule, TzName}, Daylight, ICalendar, Standard, TimeZone};

use crate::args::Args;
const PRODID: &str = "-//CMERLIN//ROSTER TO ICS//EN";
const VERSION: &str = "2.0";
const TZ_STRING: &str = "EUROPE/LONDON";
const CALENDAR_NAME: &str = "MY CALENDAR";

const STANDARD_NAME: &str = "GMT";
const STANDARD_RRULE: &str = "FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU";
const STANDARD_DT_START: &str = "19701025T020000";
const STANDARD_OFFSET_FROM: &str = "+0100";
const STANDARD_OFFSET_TO: &str = "+0000";

const DAYLIGHT_NAME: &str = "BST";
const DAYLIGHT_RRULE: &str = "FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU";
const DAYLIGHT_DT_START: &str = "19700329T010000";
const DAYLIGHT_OFFSET_FROM: &str = "+0000";
const DAYLIGHT_OFFSET_TO: &str = "+0100";

#[test]
pub fn test_ics() {
    
    

}

pub struct Calendar<'a> {
    inner: ICalendar<'a>,
    first_day_of_month: NaiveDate,
    now_string: String,
}
impl<'a> Calendar<'a> {
    pub fn new(first_day_of_month: NaiveDate, args: &Args) -> Self {
        let mut calendar = ICalendar::new(VERSION, PRODID);
        let method = Method::new("PUBLISH");
        let calscale = CalScale::new("GREGORIAN");
        let x_wr_tz = Property::new("X-WR-TIMEZONE", TZ_STRING);
        let x_wr_calname = Property::new("X-WR-CALNAME", args.name.unwrap_or(CALENDAR_NAME.into()));
        calendar.push(method);
        calendar.push(calscale);
        calendar.push(x_wr_tz);
        calendar.push(x_wr_calname);    
        let mut dl = Daylight::new(DAYLIGHT_DT_START, DAYLIGHT_OFFSET_FROM, DAYLIGHT_OFFSET_TO);
        dl.push(TzName::new(DAYLIGHT_NAME));
        dl.push(RRule::new(DAYLIGHT_RRULE));
        let mut tz = TimeZone::daylight(TZ_STRING, dl);
        tz.push(Property::new("X-LIC-LOCATION", TZ_STRING));
        let mut standard = Standard::new(STANDARD_DT_START, STANDARD_OFFSET_FROM, STANDARD_OFFSET_TO);
        standard.push(TzName::new(STANDARD_NAME));
        standard.push(RRule::new(STANDARD_RRULE));
        tz.add_standard(standard);
        calendar.add_timezone(tz);

    } 
}