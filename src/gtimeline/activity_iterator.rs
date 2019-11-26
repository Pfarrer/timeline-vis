use crate::json::Token;
use crate::model::{Activity, ActivityBuilder};
use std::iter::Peekable;

pub struct ActivityIterator<'a, It>
where
    It: Iterator<Item = Token>,
{
    tokenizer: Peekable<&'a mut It>,
}

impl<'a, It> ActivityIterator<'a, It>
where
    It: Iterator<Item = Token>,
{
    pub fn new(tokenizer: &'a mut It) -> ActivityIterator<'a, It> {
        ActivityIterator {
            tokenizer: tokenizer.peekable(),
        }
    }
}

impl<'a, It> Iterator for ActivityIterator<'a, It>
where
    It: Iterator<Item = Token>,
{
    type Item = Result<Activity, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokenizer.peek() == Some(&Token::ObjectStart) {
            self.tokenizer.next();
        } else {
            return None;
        }

        let mut builder = ActivityBuilder::new();
        loop {
            match self.tokenizer.next() {
                Some(Token::Identifier(identifier)) => match identifier.as_ref() {
                    "type" => {
                        match self.tokenizer.next().unwrap() {
                            Token::String(v) => {
                                builder.r#type(v);
                            }
                            _ => panic!("type value is not a string"),
                        };
                    }
                    "confidence" => {
                        let confidence = match self.tokenizer.next().unwrap() {
                            Token::Integer(v) => v,
                            _ => panic!("confidence value not an integer"),
                        };
                        builder.confidence(confidence as u8);
                    }
                    _ => panic!("Unexpected field {}", identifier),
                },
                Some(Token::ObjectEnd) => return Some(builder.build()),

                a => panic!("Unexpected token {:?}", a),
            }
        }
    }
}
