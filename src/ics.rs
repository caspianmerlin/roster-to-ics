use chrono::{NaiveDate, NaiveDateTime, Utc};
use guid_create::GUID;
use ics::{components::Property, parameters::{TzIDParam, Value}, properties::{Attendee, CalScale, Description, DtEnd, DtStart, Method, RRule, Sequence, Status, Summary, Trigger, TzName}, Alarm, Daylight, Event, ICalendar, Standard, TimeZone};

use crate::{roster::CalendarEvent, util::ReminderAdvance};
const PRODID: &str = "-//CMERLIN//ROSTER TO ICS//EN";
const VERSION: &str = "2.0";
const TZ_STRING: &str = "Europe/London";
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

const DATE_FORMAT_STRING: &str = "%Y%m%d";
const DATE_TIME_FORMAT_STRING: &str = "%Y%m%dT%H%M%S";
const DATE_TIME_Z_FORMAT_STRING: &str = "%Y%m%dT%H%M%SZ";


pub struct CalendarSettings {
    email_address: Option<String>,
    reminder_advance: ReminderAdvance,
    now_string: String,
}

pub fn new_calendar<'a>(calendar_name: Option<String>, email_address: Option<String>, reminder_advance: ReminderAdvance) -> (ICalendar<'a>, CalendarSettings) {
    let mut calendar = ICalendar::new(VERSION, PRODID);
    let method = Method::new("PUBLISH");
    let calscale = CalScale::new("GREGORIAN");
    let x_wr_tz = Property::new("X-WR-TIMEZONE", TZ_STRING);
    let x_wr_calname = Property::new("X-WR-CALNAME", calendar_name.unwrap_or(CALENDAR_NAME.into()));
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

    let now = Utc::now();
    let now_string = now.format(DATE_TIME_Z_FORMAT_STRING).to_string();

    let settings = CalendarSettings {
        email_address,
        reminder_advance,
        now_string
    };
    (calendar, settings)
}

pub fn new_event<'a>(settings: &'a CalendarSettings, calendar_event: &'a CalendarEvent) -> Event<'a> {

    let mut event = Event::new(GUID::rand().to_string(), &settings.now_string);

    let desc = match calendar_event {
        CalendarEvent::Normal { name, start, end } => {
            event.push(start.start_time_fmt());
            event.push(end.end_time_fmt());
            name
        },
        CalendarEvent::AllDay { name, date } => {
            event.push(date.start_date_fmt());
            name
        },
        CalendarEvent::MultiDay { name, start, end } => {
            event.push(start.start_date_fmt());
            event.push(end.end_date_fmt());
            name
        },
    };

    event.push(Sequence::new("0"));
    event.push(Summary::new(desc.clone()));
    event.push(Status::confirmed());


    if let Some(email) = &settings.email_address {
        
        let display_alarm = Alarm::display(Trigger::new(settings.reminder_advance.to_string()), Description::new(desc.clone()));
        event.add_alarm(display_alarm);

        let mut email_alarm = Alarm::email(Trigger::new(settings.reminder_advance.to_string()), Description::new(desc.clone()), Summary::new(desc.clone()));
        email_alarm.push(Attendee::new(format!("mailto:{email}")));
        event.add_alarm(email_alarm);

    }

    event
}




trait IcsDateTimeFormat {
    fn start_time_fmt(&self) -> DtStart;
    fn end_time_fmt(&self) -> DtEnd;
}

trait IcsDateFormat {
    fn start_date_fmt(&self) -> DtStart;
    fn end_date_fmt(&self) -> DtEnd;
}

impl IcsDateTimeFormat for NaiveDateTime {
    fn start_time_fmt(&self) -> DtStart {
        let mut start = DtStart::new(self.format(DATE_TIME_FORMAT_STRING).to_string());
        start.add(TzIDParam::new(TZ_STRING));
        start
    }
    fn end_time_fmt(&self) -> DtEnd {
        let mut end = DtEnd::new(self.format(DATE_TIME_FORMAT_STRING).to_string());
        end.add(TzIDParam::new(TZ_STRING));
        end
    }
}

impl IcsDateFormat for NaiveDate {
    fn start_date_fmt(&self) -> DtStart {
        let mut start = DtStart::new(self.format(DATE_FORMAT_STRING).to_string());
        start.add(Value::DATE);
        start
    }
    fn end_date_fmt(&self) -> DtEnd {
        let mut end = DtEnd::new(self.format(DATE_FORMAT_STRING).to_string());
        end.add(Value::DATE);
        end
    }
}