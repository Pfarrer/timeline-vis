use crate::json::JsonTokenizer;
use crate::json::Token;
use chrono::NaiveDateTime;
use geo::Point;
use std::iter::Peekable;
use zip::read::ZipFile;

#[derive(Debug)]
pub struct Location {
    timestamp: NaiveDateTime,
    point: Point<f32>,
    accuracy: u8,
    activities: Option<Vec<Activity>>,
}

#[derive(Debug)]
pub struct Activity {}

impl Location {
    fn new_invalid() -> Location {
        Location {
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            point: (std::f32::MAX, std::f32::MAX).into(),
            accuracy: std::u8::MAX,
            activities: None,
        }
    }

    fn is_valid(&self) -> bool {
        let invalid = Location::new_invalid();

        if self.timestamp == invalid.timestamp {
            return false;
        }

        if self.point.x() == invalid.point.x() || self.point.y() == invalid.point.y() {
            return false;
        }

        if self.accuracy > 100 {
            return false;
        }

        return true;
    }
}

pub struct LocationIterator<'a> {
    tokenizer: Peekable<JsonTokenizer<ZipFile<'a>>>,
}

impl<'a> LocationIterator<'a> {
    fn parse_location(&mut self) -> Result<Location, ()> {
        assert_eq!(self.tokenizer.next(), Some(Token::ObjectStart));

        let mut location = Location::new_invalid();
        loop {
            match self.tokenizer.next() {
                Some(Token::Identifier(identifier)) => match identifier.as_ref() {
                    "timestampMs" => {
                        location.timestamp = match self.tokenizer.next().unwrap() {
                            Token::String(v) => {
                                let secs = v.parse::<i64>().unwrap() / 1000;
                                NaiveDateTime::from_timestamp(secs, 0)
                            }
                            _ => panic!("timestampMs value not found"),
                        };
                    }
                    "latitudeE7" => {
                        let latitude_e7 = match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => v,
                            _ => panic!("latitudeE7 value not found"),
                        };
                        let lat = if latitude_e7 > 900000000 {
                            latitude_e7 - 4294967296
                        } else {
                            latitude_e7
                        } as f32
                            / 10000000.0;
                        location.point.set_y(lat);
                    }
                    "longitudeE7" => {
                        let longitude_e7 = match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => v,
                            _ => panic!("longitudeE7 value not found"),
                        };
                        let lat = if longitude_e7 > 1800000000 {
                            longitude_e7 - 4294967296
                        } else {
                            longitude_e7
                        } as f32
                            / 10000000.0;
                        location.point.set_x(lat);
                    }
                    "accuracy" => {
                        location.accuracy = match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => v as u8,
                            _ => panic!("accuracy value not found"),
                        };
                    }
                    "activity" => location.activities = self.parse_activities().ok(),
                    a => panic!("Identifier not implemented: {}", a),
                },
                Some(Token::ObjectEnd) => break,
                a => panic!("Unexpected token: {:?}", a),
            }
        }

        if location.is_valid() {
            Ok(location)
        } else {
            Err(())
        }
    }

    fn parse_activities(&mut self) -> Result<Vec<Activity>, ()> {
        assert_eq!(self.tokenizer.next(), Some(Token::ArrayStart));

        let mut activities = Vec::new();
        loop {
            match self.tokenizer.peek().unwrap() {
                Token::ObjectStart => activities.push(self.parse_activity().unwrap()),
                Token::ArrayEnd => {
                    self.tokenizer.next();
                    return Ok(activities);
                }
                a => panic!("Unexpected token: activity.{:?}", a),
            }
        }
    }

    fn parse_activity(&mut self) -> Result<Activity, ()> {
        assert_eq!(self.tokenizer.next(), Some(Token::ObjectStart));

        loop {
            match self.tokenizer.next().unwrap() {
                Token::ObjectEnd => break,
                a => panic!("Unexpected token: activity.{:?}", a),
            }
        }

        assert_eq!(self.tokenizer.next(), Some(Token::ObjectEnd));
        Err(())
    }
}

impl<'a> Iterator for LocationIterator<'a> {
    type Item = Result<Location, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokenizer.peek() {
            Some(_) => Some(self.parse_location()),
            None => None,
        }
    }
}

pub fn parse(mut tokenizer: JsonTokenizer<ZipFile>) -> LocationIterator {
    consume_irrelevant_json_tokens(&mut tokenizer);
    LocationIterator {
        tokenizer: tokenizer.peekable(),
    }
}

fn consume_irrelevant_json_tokens<T>(tokenizer: &mut T)
where
    T: Iterator<Item = Token>,
{
    assert_eq!(tokenizer.next(), Some(Token::ObjectStart));
    assert_eq!(
        tokenizer.next(),
        Some(Token::Identifier("locations".into()))
    );
    assert_eq!(tokenizer.next(), Some(Token::ArrayStart));
}
