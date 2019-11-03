use crate::model::Activity;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Classification {
    timestamp_ms: NaiveDateTime,
    activities: Vec<Activity>,
}

pub struct ClassificationBuilder {
    invalid: bool,
    timestamp_ms: Option<NaiveDateTime>,
    activities: Option<Vec<Activity>>,
}

impl ClassificationBuilder {
    pub fn new() -> ClassificationBuilder {
        ClassificationBuilder {
            invalid: false,
            timestamp_ms: None,
            activities: None,
        }
    }

    pub fn timestamp(&mut self, seconds: i64) -> &mut ClassificationBuilder {
        self.timestamp_ms = Some(NaiveDateTime::from_timestamp(seconds, 0));
        self
    }

    pub fn activities(&mut self, activities: Vec<Activity>) -> &mut ClassificationBuilder {
        self.activities = Some(activities);
        self
    }

    pub fn build(self) -> Result<Classification, ()> {
        if !self.invalid && self.timestamp_ms.is_some() && self.activities.is_some() {
            Ok(Classification {
                timestamp_ms: self.timestamp_ms.unwrap(),
                activities: self.activities.unwrap(),
            })
        } else {
            Err(())
        }
    }
}
