use chrono::{DateTime, Utc, TimeZone};
use serde::{self, Deserialize, Deserializer};

const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
{
    let mut dates: Vec<String> = Vec::deserialize(deserializer)?;
    Utc.datetime_from_str(dates.pop().expect("Date is missing.").as_str(), FORMAT).map_err(serde::de::Error::custom)
}
