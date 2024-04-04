use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer};

pub(crate) fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").map_err(de::Error::custom)
}
