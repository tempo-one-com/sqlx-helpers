use sqlx::types::chrono::{NaiveDate, NaiveDateTime, NaiveTime};

///format attendu: yyyy-mm-dd
pub fn parse_iso_to_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}

///format attendu: HH:mm:ss
pub fn parse_iso_to_time(value: &str) -> Option<NaiveTime> {
    let mut time = String::from(value);

    if value.len() == 5 {
        time.push_str(":00");
    }

    NaiveTime::parse_from_str(&time, "%H:%M:%S").ok()
}

pub fn parse_teliway_to_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y%m%d").ok()
}

pub fn parse_teliway_opt_to_date(value: Option<&str>) -> Option<NaiveDate> {
    match value {
        Some(x) => parse_teliway_to_date(x),
        _ => None,
    }
}

pub fn parse_teliway_to_time(value: &str) -> Option<NaiveTime> {
    if value.len() == 6 {
        NaiveTime::parse_from_str(value, "%H%M%S").ok()
    } else if value.len() == 4 {
        NaiveTime::parse_from_str(value, "%H%M").ok()
    } else {
        None
    }
}

pub fn parse_teliway_opt_to_time(value: Option<&str>) -> Option<NaiveTime> {
    match value {
        Some(x) => parse_teliway_to_time(x),
        _ => None,
    }
}

pub fn parse_teliway_date_time(date: &str, time: &str) -> Option<NaiveDateTime> {
    parse_teliway_date_time_opt(Some(date), Some(time))
}

pub fn parse_teliway_date_time_opt(
    date: Option<&str>,
    time: Option<&str>,
) -> Option<NaiveDateTime> {
    if let Some(date) = parse_teliway_opt_to_date(date) {
        if let Some(time) = parse_teliway_opt_to_time(time) {
            Some(NaiveDateTime::new(date, time))
        } else {
            Some(NaiveDateTime::new(date, NaiveTime::default()))
        }
    } else {
        None
    }
}
