use chrono::{DateTime, Utc, TimeZone};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT_INPUT: &str = "%Y-%m-%dT%H:%M:%S";

const FORMAT_OUTPUT: &str = "%d-%m-%Y, %A at %H:%M";

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
{
    let mut dates: Vec<String> = Vec::deserialize(deserializer)?;
    Utc.datetime_from_str(dates.pop().expect("Date is missing.").as_str(), FORMAT_INPUT).map_err(serde::de::Error::custom)
}

pub fn serialize<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&format!("{}", date.format(FORMAT_OUTPUT)))
}
