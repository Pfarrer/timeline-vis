use crate::model::Activity;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Classification {
    pub timestamp: NaiveDateTime,
    pub activities: Vec<Activity>,
}

pub struct ClassificationBuilder {
    timestamp_ms: Option<NaiveDateTime>,
    activities: Option<Vec<Activity>>,
}

impl ClassificationBuilder {
    pub fn new() -> ClassificationBuilder {
        ClassificationBuilder {
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

    pub fn build(self) -> Result<Classification, String> {
        let mut errors: Vec<String> = Vec::new();
        if self.timestamp_ms.is_none() {
            errors.push("classification.timestamp_ms missing".into());
        }
        if self.activities.is_none() {
            errors.push("classification.activities missing".into());
        }

        if errors.len() == 0 {
            Ok(Classification {
                timestamp: self.timestamp_ms.unwrap(),
                activities: self.activities.unwrap(),
            })
        } else {
            Err(errors.join("; "))
        }
    }
}
