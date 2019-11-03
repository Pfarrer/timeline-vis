use crate::gtimeline::classification_iterator::ClassificationIterator;
use crate::json::Token;
use crate::model::{Location, LocationBuilder};

pub struct LocationIterator<It>
where
    It: Iterator<Item = Token>,
{
    tokenizer: It,
}

impl<It> LocationIterator<It>
where
    It: Iterator<Item = Token>,
{
    pub fn new(tokenizer: It) -> LocationIterator<It> {
        LocationIterator { tokenizer }
    }
}

impl<It> Iterator for LocationIterator<It>
where
    It: Iterator<Item = Token>,
{
    type Item = Result<Location, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        assert_eq!(self.tokenizer.next(), Some(Token::ObjectStart));

        let mut builder = LocationBuilder::new();
        loop {
            match self.tokenizer.next() {
                Some(Token::Identifier(identifier)) => match identifier.as_ref() {
                    "timestampMs" => {
                        match self.tokenizer.next().unwrap() {
                            Token::String(v) => {
                                let secs = v.parse::<i64>().unwrap() / 1000;
                                builder.timestamp(secs);
                            }
                            _ => panic!("timestampMs value is not a string"),
                        };
                    }
                    "latitudeE7" => {
                        let latitude_e7 = match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => v,
                            _ => panic!("latitudeE7 value not an integer"),
                        };
                        let lat = if latitude_e7 > 900000000 {
                            latitude_e7 - 4294967296
                        } else {
                            latitude_e7
                        } as f32
                            / 10000000.0;
                        builder.point_y(lat);
                    }
                    "longitudeE7" => {
                        let longitude_e7 = match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => v,
                            _ => panic!("longitudeE7 value not an integer"),
                        };
                        let lat = if longitude_e7 > 1800000000 {
                            longitude_e7 - 4294967296
                        } else {
                            longitude_e7
                        } as f32
                            / 10000000.0;
                        builder.point_x(lat);
                    }
                    "accuracy" => {
                        match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => builder.accuracy(v),
                            _ => panic!("accuracy value not an integer"),
                        };
                    }
                    "activity" => {
                        assert_eq!(self.tokenizer.next(), Some(Token::ArrayStart));

                        {
                            let classifications = ClassificationIterator::new(&mut self.tokenizer)
                                .filter_map(|it| it.ok())
                                .collect();
                            builder.classifications(classifications);
                        }

                        //                        assert_eq!(self.tokenizer.next(), Some(Token::ArrayEnd));
                    }
                    _ => panic!("Unexpected field {}", identifier),
                },
                Some(Token::ObjectEnd) => return Some(builder.build()),

                a => panic!("Unexpected token {:?}", a),
            }
        }
    }
}
