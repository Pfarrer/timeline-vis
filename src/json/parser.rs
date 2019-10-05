use crate::json::Token;
use std::collections::HashMap;

pub struct LazyParser<'a, T> {
    tokenizer: Box<dyn Iterator<Item = Token> + 'a>,
    parse_fn: Box<dyn ParseFn<'a, T> + 'a>,
}

impl<'a, T: 'a> LazyParser<'a, T> {
    pub fn new(
        tokenizer: Box<dyn Iterator<Item = Token> + 'a>,
        parse_fn: Box<dyn ParseFn<'a, T> + 'a>,
    ) -> LazyParser<'a, T> {
        LazyParser {
            tokenizer: Box::new(tokenizer),
            parse_fn: parse_fn,
        }
    }
}

impl<'a, 'p, T> Iterator for LazyParser<'a, T> {
    type Item = Result<T, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_fn.next(&mut self.tokenizer)
    }
}

pub trait ParseFn<'a, T> {
    fn next(
        &mut self,
        tokenizer: &mut Box<dyn Iterator<Item = Token> + 'a>,
    ) -> Option<Result<T, ()>>;
}

pub struct ExpectObject<'a, T> {
    pub rules: HashMap<String, Box<dyn ParseFn<'a, T>>>,
}
impl<'a, T> ParseFn<'a, T> for ExpectObject<'a, T> {
    fn next(
        &mut self,
        tokenizer: &mut Box<dyn Iterator<Item = Token> + 'a>,
    ) -> Option<Result<T, ()>> {
        if tokenizer.next() != Some(Token::ObjectStart) {
            return Some(Err(()));
        }

        if tokenizer.next() != Some(Token::ObjectEnd) {
            return Some(Err(()));
        }

        None
    }
}

pub struct ExpectArray<'a, T> {
    pub rule: Box<dyn ParseFn<'a, T>>,
}
impl<'a, T> ParseFn<'a, T> for ExpectArray<'a, T> {
    fn next(
        &mut self,
        tokenizer: &mut Box<dyn Iterator<Item = Token> + 'a>,
    ) -> Option<Result<T, ()>> {
        if tokenizer.next() != Some(Token::ArrayStart) {
            return Some(Err(()));
        }

        if tokenizer.next() != Some(Token::ArrayEnd) {
            return Some(Err(()));
        }

        None
    }
}
