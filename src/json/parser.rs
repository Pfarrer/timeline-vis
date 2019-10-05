use crate::json::Token;

pub struct LazyParser<'a, 'p, T> {
    tokenizer: Box<dyn Iterator<Item = Token> + 'a>,
    parse_fn: Box<dyn ParseFn<'p, T>>,
}

impl<'a, 'p, T: 'a> LazyParser<'a, 'p, T> {
    pub fn new(
        tokenizer: Box<dyn Iterator<Item = Token> + 'a>,
        parse_fn: Box<dyn ParseFn<'p, T>>,
    ) -> LazyParser<'a, 'p, T> {
        LazyParser {
            tokenizer: Box::new(tokenizer),
            parse_fn: parse_fn,
        }
    }
}

impl<'a, 'p, T> Iterator for LazyParser<'a, 'p, T>
where 'p: 'a {
    type Item = Result<T, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_fn.next(self.tokenizer)
    }
}

pub trait ParseFn<'a, T> {
    fn next(&mut self, tokenizer: Box<dyn Iterator<Item = Token> + 'a>) -> Option<Result<T, ()>>;
}

pub struct ExpectObject<'a, T> {
    pub rules: std::collections::HashMap<String, Box<dyn ParseFn<'a, T>>>, // createType: Box<dyn Fn() -> T>,
}
impl<'a, T> ParseFn<'a, T> for ExpectObject<'a, T> {
    fn next(
        &mut self,
        mut tokenizer: Box<dyn Iterator<Item = Token> + 'a>,
    ) -> Option<Result<T, ()>> {
        assert_eq!(tokenizer.next(), Some(Token::ObjectStart));
        assert_eq!(tokenizer.next(), Some(Token::ObjectEnd));
        None
    }
}
