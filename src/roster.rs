use std::fmt::Display;

use chrono::{Datelike, Days, NaiveDate, NaiveDateTime};

// M       0630-1330
// D1      0800-1530
// D2        0900-1730
// D3        1000-1830
// D4      1530-2300*
// D5        1515-2345
// A        1330-2200
// A1      1330-2100
// N       2200-0630

#[derive(Debug)]
pub enum EventType {
    M,
    Mx,
    Mt,

    D1,
    D1t,

    D2,
    D2t,

    D3,
    D3t,

    D4,
    D4t,

    D5,
    D5t,

    A,
    At,

    A1,
    A1t,

    N,

    DayInLieu,
    Leave,
    Sick,
    DayOff,
    Other { name: String, hour_start: u32, minute_start: u32, hour_end: u32, minute_end: u32 },
}
impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::M => write!(f, "M shift"),
            EventType::Mx => write!(f, "Mx shift"),
            EventType::Mt => write!(f, "MT shift"),
            EventType::D1 => write!(f, "D1 shift"),
            EventType::D1t => write!(f, "D1T shift"),
            EventType::D2 => write!(f, "D2 shift"),
            EventType::D2t => write!(f, "D2T shift"),
            EventType::D3 => write!(f, "D3 shift"),
            EventType::D3t => write!(f, "D3T shift"),
            EventType::D4 => write!(f, "D4 shift"),
            EventType::D4t => write!(f, "D4T shift"),
            EventType::D5 => write!(f, "D5 shift"),
            EventType::D5t => write!(f, "D5T shift"),
            EventType::A => write!(f, "A shift"),
            EventType::At => write!(f, "AT shift"),
            EventType::A1 => write!(f, "A1 shift"),
            EventType::A1t => write!(f, "A1T shift"),
            EventType::N => write!(f, "N shift"),
            EventType::DayInLieu => write!(f, "DIL"),
            EventType::Leave => write!(f, "Annual leave"),
            EventType::Sick => write!(f, "Sick leave"),
            EventType::DayOff => write!(f, "Day off"),
            EventType::Other { name, .. } => write!(f, "{name}"),
        }
    }
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            return EventType::DayOff;
        }
        match s.to_lowercase().as_str() {
            "m" => EventType::M,
            "mx" => EventType::Mx,
            "mt" => EventType::Mt,
            "d1" => EventType::D1,
            "d1t" => EventType::D1t,
            "d2" => EventType::D2,
            "d2t" => EventType::D2t,
            "d3" => EventType::D3,
            "d3t" => EventType::D3t,
            "d4" => EventType::D4,
            "d4t" => EventType::D4t,
            "d5" => EventType::D5,
            "d5t" => EventType::D5t,
            "a" => EventType::A,
            "at" => EventType::At,
            "a1" => EventType::A1,
            "a1t" => EventType::A1t,
            "n" => EventType::N,
            "dil" => EventType::DayInLieu,
            "al" => EventType::Leave,
            "sc" | "ssc" => EventType::Sick,
            "//" | "s" => EventType::DayOff,
            _ => EventType::Other { name: s.to_string(), hour_start: 0, minute_start: 0, hour_end: 0, minute_end: 0 },
            
        }
    }
}

impl EventType {
    pub fn start_and_end_time(&self, summer: bool) -> Option<(u32, u32, u32, u32)> {
        match self {
            EventType::M        => Some((06, 30, 13, 30)),
            EventType::Mx       => Some((06, 30, 14, 00)),
            EventType::Mt       => Some((06, 30, 13, 30)),
            EventType::D1       => Some((08, 00, 15, 30)),
            EventType::D1t      => Some((08, 00, 15, 30)),
            EventType::D2       => Some((09, 00, 17, 30)),
            EventType::D2t      => Some((09, 00, 17, 30)),
            EventType::D3       => Some((10, 00, 18, 30)),
            EventType::D3t      => Some((10, 00, 18, 30)),
            EventType::D4       => if summer { Some((15, 30, 23, 00)) } else { Some((15, 00, 22, 30)) },
            EventType::D4t      => if summer { Some((15, 30, 23, 00)) } else { Some((15, 00, 22, 30)) },
            EventType::D5       => Some((15, 15, 23, 45)),
            EventType::D5t      => Some((15, 15, 23, 45)),
            EventType::A        => Some((13, 30, 22, 00)),
            EventType::At       => Some((13, 30, 22, 00)),
            EventType::A1       => Some((13, 30, 21, 00)),
            EventType::A1t      => Some((13, 30, 21, 00)),
            EventType::DayInLieu => None,
            EventType::N        => Some((22, 00, 06, 30)),
            EventType::Leave    => None,
            EventType::Sick     => None,
            EventType::DayOff   => None,
            EventType::Other { hour_start, minute_start, hour_end, minute_end, .. } => Some((*hour_start, *minute_start, *hour_end, *minute_end)),
            
        }
    }
}

#[derive(Debug)]
pub enum CalendarEvent {
    Normal { name: String, start: NaiveDateTime, end: NaiveDateTime },
    AllDay { name: String, date: NaiveDate },
    MultiDay { name: String, start: NaiveDate, end: NaiveDate },
}


pub fn generate_calendar_events(first_day_of_month: NaiveDate, days: Vec<EventType>) -> Vec<CalendarEvent>{
    let mut events = Vec::new();
    let mut days_off_or_leave_started = None;
    let mut leave_polluted = false;

    // Is it summer? April - October inclusive
    let summer = (4..11).contains(&first_day_of_month.month());

    for (i, day) in days.iter().enumerate() {

        let mut today_is_day_off_or_leave = false;

        
        

        // If we can generate a start and end_time, do that
        if let Some((hour_start, min_start, hour_end, min_end)) = day.start_and_end_time(summer) {
            let start = first_day_of_month.checked_add_days(Days::new(i as u64)).unwrap().and_hms_opt(hour_start, min_start, 0).unwrap();
            let mut end = first_day_of_month.checked_add_days(Days::new(i as u64)).unwrap().and_hms_opt(hour_end, min_end, 0).unwrap();

            // If it's a night shift, we finish on the following day
            if let EventType::N = day {
                end = end.checked_add_days(Days::new(1)).unwrap();
            }
            events.push(CalendarEvent::Normal { name: day.to_string(), start, end });
        }

        // If it's annual leave, deal with that
        else if let EventType::Leave = day {
            today_is_day_off_or_leave = true;
            leave_polluted = true;
            if days_off_or_leave_started.is_none() {
                days_off_or_leave_started = Some(first_day_of_month.checked_add_days(Days::new(i as u64)).unwrap());
            }
        }

        // If it's not annual leave and not a normal event, it's an all-day event
        else {
            // If it's a day off, deal with that. We don't record them unless they are leave-adjacent
            if let EventType::DayOff = day {
                today_is_day_off_or_leave = true;
                if days_off_or_leave_started.is_none() {
                    days_off_or_leave_started = Some(first_day_of_month.checked_add_days(Days::new(i as u64)).unwrap());
                }
            }
            else {
                events.push(CalendarEvent::AllDay { name: day.to_string(), date: first_day_of_month.checked_add_days(Days::new(i as u64)).unwrap() });
            }
        }

        // Now, if today was not leave but we have a leave start date, it means it ended yesterday.
        if !today_is_day_off_or_leave && days_off_or_leave_started.is_some() {
            let start_date = days_off_or_leave_started.take().unwrap();
            if leave_polluted {
                leave_polluted = false;
                
                let end_date = first_day_of_month.checked_add_days(Days::new((i - 1) as u64)).unwrap();
                // If they're the same day, we'll put it as an all-day event. Otherwise, as a multi-day event.
                if start_date == end_date {
                    events.push(CalendarEvent::AllDay { name: String::from("Annual leave"), date: start_date });
                } else {
                    events.push(CalendarEvent::MultiDay { name: String::from("Annual leave"), start: start_date, end: end_date.checked_add_days(Days::new(1)).unwrap() });
                }
            }
            
        }

    }

    if days_off_or_leave_started.is_some() && leave_polluted {
        let start_date = days_off_or_leave_started.take().unwrap();
        let end_date = first_day_of_month.checked_add_days(Days::new((days.len() - 1) as u64)).unwrap();
        // If they're the same day, we'll put it as an all-day event. Otherwise, as a multi-day event.
        if start_date == end_date {
            events.push(CalendarEvent::AllDay { name: String::from("Annual leave"), date: start_date });
        } else {
            events.push(CalendarEvent::MultiDay { name: String::from("Annual leave"), start: start_date, end: end_date.checked_add_days(Days::new(1)).unwrap() });
        }
    }

    events
}

#[test]
fn test_here() {
    
}