use chrono::DateTime;

pub trait StringToTimemillis {
    fn to_timestamp(&self) -> Option<DateTime<chrono::Utc>>;
}

impl StringToTimemillis for String {
    fn to_timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        self.parse::<i64>()
            .ok()
            .and_then(chrono::DateTime::from_timestamp_millis)
    }
}

impl StringToTimemillis for i64 {
    fn to_timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        chrono::DateTime::from_timestamp_millis(*self)
    }
}

impl StringToTimemillis for Option<i64> {
    fn to_timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        self.and_then(chrono::DateTime::from_timestamp_millis)
    }
}

impl StringToTimemillis for Option<String> {
    fn to_timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        self.as_ref()
            .and_then(|dob| dob.parse::<i64>().ok())
            .and_then(chrono::DateTime::from_timestamp_millis)
    }
}
impl StringToTimemillis for &str {
    fn to_timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        self.to_string()
            .parse::<i64>()
            .ok()
            .and_then(chrono::DateTime::from_timestamp_millis)
    }
}
