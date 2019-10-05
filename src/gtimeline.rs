use crate::json::parser;
use crate::json::JsonTokenizer;
use chrono::NaiveDateTime;
use geo::Point;
use std::collections::HashMap;
use zip::read::ZipFile;

#[derive(Debug)]
pub struct Location {
    timestamp: NaiveDateTime,
    point: Point<f32>,
    accuracy: u8,
}

impl Location {
    fn new_invalid() -> Location {
        Location {
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            point: (std::f32::MAX, std::f32::MAX).into(),
            accuracy: std::u8::MAX,
            // activities: None,
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

// pub struct LocationIterator<'a> {
// }

// impl<'a> LocationIterator<'a> {
//     fn parse_location(&mut self) -> Result<Location, ()> {
//         assert_eq!(self.tokenizer.next(), Some(Token::ObjectStart));

//         let mut location = Location::new_invalid();
//         loop {
//             match self.tokenizer.next() {
//                 Some(Token::Identifier(identifier)) => match identifier.as_ref() {
//                     "timestampMs" => {
//                         location.timestamp = match self.tokenizer.next().unwrap() {
//                             Token::String(v) => {
//                                 let secs = v.parse::<i64>().unwrap() / 1000;
//                                 NaiveDateTime::from_timestamp(secs, 0)
//                             }
//                             _ => panic!("timestampMs value not found"),
//                         };
//                     }
//                     "latitudeE7" => {
//                         let latitude_e7 = match self.tokenizer.next().unwrap() {
//                             Token::Integer(v) => v,
//                             _ => panic!("latitudeE7 value not found"),
//                         };
//                         let lat = if latitude_e7 > 900000000 {
//                             latitude_e7 - 4294967296
//                         } else {
//                             latitude_e7
//                         } as f32
//                             / 10000000.0;
//                         location.point.set_y(lat);
//                     }
//                     "longitudeE7" => {
//                         let longitude_e7 = match self.tokenizer.next().unwrap() {
//                             Token::Integer(v) => v,
//                             _ => panic!("longitudeE7 value not found"),
//                         };
//                         let lat = if longitude_e7 > 1800000000 {
//                             longitude_e7 - 4294967296
//                         } else {
//                             longitude_e7
//                         } as f32
//                             / 10000000.0;
//                         location.point.set_x(lat);
//                     }
//                     "accuracy" => {
//                         location.accuracy = match self.tokenizer.next().unwrap() {
//                             Token::Integer(v) => v as u8,
//                             _ => panic!("accuracy value not found"),
//                         };
//                     }
//                     // "activity" => location.activities = self.parse_activities().ok(),
//                     a => panic!("Identifier not implemented: {}", a),
//                 },
//                 Some(Token::ObjectEnd) => break,
//                 a => panic!("Unexpected token: {:?}", a),
//             }
//         }

//         if location.is_valid() {
//             Ok(location)
//         } else {
//             Err(())
//         }
//     }
// }

// impl<'a> Iterator for LocationIterator<'a> {
//     type Item = Result<Location, ()>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.tokenizer.peek() {
//             Some(_) => Some(self.parse_location()),
//             None => None,
//         }
//     }
//

pub fn parse<'a>(
    tokenizer: JsonTokenizer<ZipFile<'a>>,
) -> impl Iterator<Item = Result<Location, ()>> + 'a {
    let rules = parser::ExpectObject {
        rules: HashMap::new(),
    };
    parser::LazyParser::new(Box::new(tokenizer), Box::new(rules))
}
// create: Box::new(|| Ok(Location::new_invalid()))
