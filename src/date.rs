use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Date {
    NaiveDate(NaiveDate),
    NaiveDateTime(NaiveDateTime),
    DateTimeUtc(DateTime<Utc>),
    DateTimeFixedOffset(String), // change to parse with string instead
}
