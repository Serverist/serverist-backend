use chrono::{DateTime, Utc, TimeZone};
use uuid::Uuid;

#[derive(PartialEq, Eq, Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: Uuid) -> UserId {
        UserId(id)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct IconId(Uuid);

impl IconId {
    pub fn new(id: Uuid) -> IconId {
        IconId(id)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct PastDateTime(DateTime<Utc>);

pub struct PastDateTimeError;

impl PastDateTime {
    pub fn new<Tz: TimeZone>(date_time: DateTime<Tz>) -> Result<Self, PastDateTimeError> {
        let millis = date_time.timestamp_millis();
        if (millis - Utc::now().timestamp_millis()) > 0 {
            Err(PastDateTimeError)
        } else {
            Ok(Self(Utc.timestamp_millis(millis)))
        }
    }
}