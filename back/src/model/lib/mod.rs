use std::{collections::HashSet, hash::Hash};

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

pub fn vec_diff<T: Clone + Eq + Hash>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let set1: HashSet<T> = vec1.into_iter().collect();
    let set2: HashSet<T> = vec2.into_iter().collect();
    (&set1 - &set2).into_iter().collect()
}
