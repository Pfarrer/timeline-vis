use chrono::NaiveDateTime;
use geo::{Coordinate, Point};

use crate::model::Classification;

#[derive(Debug)]
pub struct Location {
    timestamp: NaiveDateTime,
    point: Point<f32>,
    accuracy: u8,
    activity_classifications: Vec<Classification>,
}

pub struct LocationBuilder {
    invalid: bool,
    timestamp: Option<NaiveDateTime>,
    point_x: Option<f32>,
    point_y: Option<f32>,
    accuracy: Option<u8>,
    classifications: Option<Vec<Classification>>,
}

impl LocationBuilder {
    pub fn new() -> LocationBuilder {
        LocationBuilder {
            invalid: false,
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
        self.accuracy = Some(val as u8);
        self
    }

    pub fn classifications(
        &mut self,
        activity_classifications: Vec<Classification>,
    ) -> &mut LocationBuilder {
        self.classifications = Some(activity_classifications);
        self
    }

    pub fn build(self) -> Result<Location, ()> {
        if !self.invalid
            && self.timestamp.is_some()
            && self.point_x.is_some()
            && self.point_y.is_some()
            && self.accuracy.is_some()
        {
            Ok(Location {
                timestamp: self.timestamp.unwrap(),
                point: Point(Coordinate {
                    x: self.point_x.unwrap(),
                    y: self.point_y.unwrap(),
                }),
                accuracy: self.accuracy.unwrap(),
                activity_classifications: self.classifications.unwrap_or(vec![]),
            })
        } else {
            Err(())
        }
    }
}
