use chrono::{DateTime, Local, Utc};
use sqlx::types::{ipnetwork::IpNetwork, time::OffsetDateTime};

pub trait OffsetDateTimeToDateTime {
    fn to_datetime_utc(&self) -> DateTime<Utc>;
}

impl OffsetDateTimeToDateTime for OffsetDateTime {
    fn to_datetime_utc(&self) -> DateTime<Utc> {
        let from = self.unix_timestamp() * 1000;
        DateTime::from_timestamp_millis(from).unwrap_or_else(|| Local::now().to_utc())
    }
}

impl OffsetDateTimeToDateTime for Option<OffsetDateTime> {
    fn to_datetime_utc(&self) -> DateTime<Utc> {
        match self {
            Some(offset_dt) => {
                let from = offset_dt.unix_timestamp() * 1000;
                DateTime::from_timestamp_millis(from).unwrap_or_else(|| Local::now().to_utc())
            }
            None => Local::now().to_utc(),
        }
    }
}

pub trait DateTimeToOffsetDateTime {
    fn to_offset_datetime(&self) -> OffsetDateTime;
}

impl DateTimeToOffsetDateTime for DateTime<Utc> {
    fn to_offset_datetime(&self) -> OffsetDateTime {
        let from = self.timestamp_millis();
        match OffsetDateTime::from_unix_timestamp(from) {
            Ok(offset_dt) => offset_dt,
            Err(_) => match OffsetDateTime::from_unix_timestamp(Utc::now().timestamp_millis()) {
                Ok(offset_dt) => offset_dt,
                Err(_) => OffsetDateTime::from_unix_timestamp(0).unwrap(),
            },
        }
    }
}

impl DateTimeToOffsetDateTime for Option<DateTime<Utc>> {
    fn to_offset_datetime(&self) -> OffsetDateTime {
        match self {
            Some(dt) => {
                let from = dt.timestamp_millis();
                match OffsetDateTime::from_unix_timestamp(from) {
                    Ok(offset_dt) => offset_dt,
                    Err(_) => {
                        match OffsetDateTime::from_unix_timestamp(Utc::now().timestamp_millis()) {
                            Ok(offset_dt) => offset_dt,
                            Err(_) => OffsetDateTime::from_unix_timestamp(0).unwrap(),
                        }
                    }
                }
            }
            None => OffsetDateTime::from_unix_timestamp(Utc::now().timestamp_millis()).unwrap(),
        }
    }
}

pub trait TimeMilisToDateTime {
    fn to_datetime_utc(&self) -> DateTime<Utc>;
    fn to_offset_datetime(&self) -> OffsetDateTime;
}

impl TimeMilisToDateTime for i64 {
    fn to_datetime_utc(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(*self).unwrap_or_else(|| Local::now().to_utc())
    }

    fn to_offset_datetime(&self) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp_nanos((*self * 1_000_000) as i128).unwrap()
    }
}

impl TimeMilisToDateTime for Option<i64> {
    fn to_datetime_utc(&self) -> DateTime<Utc> {
        match self {
            Some(time_milis) => DateTime::from_timestamp_millis(*time_milis)
                .unwrap_or_else(|| Local::now().to_utc()),
            None => Local::now().to_utc(),
        }
    }

    fn to_offset_datetime(&self) -> OffsetDateTime {
        match self {
            Some(time_milis) => {
                OffsetDateTime::from_unix_timestamp_nanos((*time_milis * 1_000_000) as i128)
                    .unwrap()
            }
            None => OffsetDateTime::from_unix_timestamp(0).unwrap(),
        }
    }
}

pub trait StringToInet {
    fn to_inet(&self) -> Option<IpNetwork>;
}

impl StringToInet for String {
    fn to_inet(&self) -> Option<IpNetwork> {
        self.parse().ok()
    }
}

impl StringToInet for Option<String> {
    fn to_inet(&self) -> Option<IpNetwork> {
        match self {
            Some(ip) => ip.parse().ok(),
            None => None,
        }
    }
}

pub trait InetToString {
    fn to_string(&self) -> String;
}

impl InetToString for IpNetwork {
    fn to_string(&self) -> String {
        self::IpNetwork::ip(&self).to_string()
        
    }
}

impl InetToString for Option<IpNetwork> {
    fn to_string(&self) -> String {
        match self {
            Some(ip) => IpNetwork::ip(&ip).to_string(),
            None => String::new(),
        }
    }
}
