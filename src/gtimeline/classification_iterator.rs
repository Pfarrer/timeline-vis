use crate::gtimeline::activity_iterator::ActivityIterator;
use crate::json::Token;
use crate::model::{Classification, ClassificationBuilder};
use std::iter::Peekable;

pub struct ClassificationIterator<'a, It>
where
    It: Iterator<Item = Token>,
{
    tokenizer: Peekable<&'a mut It>,
}

impl<'a, It> ClassificationIterator<'a, It>
where
    It: Iterator<Item = Token>,
{
    pub fn new(tokenizer: &'a mut It) -> ClassificationIterator<'a, It> {
        ClassificationIterator {
            tokenizer: tokenizer.peekable(),
        }
    }
}

impl<'a, It> Iterator for ClassificationIterator<'a, It>
where
    It: Iterator<Item = Token>,
{
    type Item = Result<Classification, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokenizer.peek() == Some(&Token::ObjectStart) {
            self.tokenizer.next();
        } else {
            return None;
        }

        let mut builder = ClassificationBuilder::new();
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
                    "activity" => {
                        assert_eq!(self.tokenizer.next(), Some(Token::ArrayStart));

                        {
                            let activities = ActivityIterator::new(&mut self.tokenizer)
                                .filter_map(|it| it.ok())
                                .collect();
                            builder.activities(activities);
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
