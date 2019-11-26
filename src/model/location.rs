use chrono::{NaiveDateTime, Utc, DateTime, Datelike, Timelike};
use serde::Serialize;

use crate::model::Classification;
use elastic::types::geo::point::GeoPoint;
use elastic::types::geo::point::mapping::DefaultGeoPointMapping;
use elastic::params::Index;
use elastic::prelude::StaticIndex;
use elastic::types::date::Date;
use elastic::types::date::mapping::DefaultDateMapping;

#[derive(Debug, Serialize, ElasticType)]
#[elastic(crate_root = "elastic::types")]
#[elastic(
    index = "timeline"
)]
pub struct Location {
    pub timestamp: Date<DefaultDateMapping>,
    pub coordinates: GeoPoint<DefaultGeoPointMapping>,
    pub accuracy: i32,
//    pub activity_classifications: Vec<Classification>,
}

pub struct LocationBuilder {
    errors: Vec<String>,
    timestamp: Option<NaiveDateTime>,
    point_x: Option<f32>,
    point_y: Option<f32>,
    accuracy: Option<u8>,
    classifications: Option<Vec<Classification>>,
}

impl LocationBuilder {
    pub fn new() -> LocationBuilder {
        LocationBuilder {
            errors: Vec::new(),
            timestamp: None,
            point_x: None,
            point_y: None,
            accuracy: None,
            classifications: None,
        }
    }

    pub fn timestamp(&mut self, seconds: i64) -> &mut LocationBuilder {
        self.timestamp = Some(NaiveDateTime::from_timestamp(seconds, 0));
        self
    }

    pub fn point_x(&mut self, val: f32) -> &mut LocationBuilder {
        self.point_x = Some(val);
        self
    }
    pub fn point_y(&mut self, val: f32) -> &mut LocationBuilder {
        self.point_y = Some(val);
        self
    }

    pub fn accuracy(&mut self, val: i64) -> &mut LocationBuilder {
        if val < 0 || val > 10000 {
            self.errors
                .push(format!("location.accuracy out of range ({})", val));
            self.accuracy = Some(0);
        } else {
            self.accuracy = Some(val as u8);
        }
        self
    }

    pub fn classifications(
        &mut self,
        activity_classifications: Vec<Classification>,
    ) -> &mut LocationBuilder {
        self.classifications = Some(activity_classifications);
        self
    }

    pub fn build(mut self) -> Result<Location, String> {
        if self.timestamp.is_none() {
            self.errors.push("location.timestamp missing".into());
        }
        if self.point_x.is_none() {
            self.errors.push("location.point_x missing".into());
        }
        if self.point_y.is_none() {
            self.errors.push("location.point_y missing".into());
        }
        if self.accuracy.is_none() {
            self.errors.push("location.accuracy missing".into());
        }

        if self.errors.len() == 0 {
            let date_time = self.timestamp.unwrap();
            Ok(Location {
                timestamp: Date::build(
                    date_time.year(),
                    date_time.month(),
                    date_time.day(),
                    date_time.hour(),
                    date_time.minute(),
                    date_time.second(),
                    date_time.timestamp_subsec_millis()
                ),
                coordinates: GeoPoint::build(
                    self.point_x.unwrap() as f64,
                    self.point_y.unwrap() as f64
                ),
                accuracy: self.accuracy.unwrap() as i32,
//                activity_classifications: self.classifications.unwrap_or(Vec::with_capacity(0)),
            })
        } else {
            Err(self.errors.join("; "))
        }
    }
}
