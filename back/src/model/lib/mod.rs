use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use ulid::Ulid;

pub fn get_new_id() -> String {
    Ulid::new().to_string()
}

pub fn get_current_date_time() -> DateTime<Utc> {
    Utc::now()
}

pub fn start_of_date(date: NaiveDate) -> DateTime<FixedOffset> {
    let offset = FixedOffset::east_opt(9 * 60 * 60).unwrap();
    DateTime::<FixedOffset>::from_local(date.and_hms(0, 0, 0), offset)
}

pub fn end_of_date(date: NaiveDate) -> DateTime<FixedOffset> {
    let offset = FixedOffset::east_opt(9 * 60 * 60).unwrap();
    DateTime::<FixedOffset>::from_local(date.and_hms(23, 59, 59), offset)
}
