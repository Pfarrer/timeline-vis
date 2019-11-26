mod activity_iterator;
mod classification_iterator;
mod location_iterator;

use crate::json::Token;
use location_iterator::LocationIterator;

pub fn parse_locations<It>(mut tokenizer: It) -> LocationIterator<It>
where
    It: Iterator<Item = Token>,
{
    assert_eq!(tokenizer.next(), Some(Token::ObjectStart));
    assert_eq!(
        tokenizer.next(),
        Some(Token::Identifier("locations".into()))
    );
    assert_eq!(tokenizer.next(), Some(Token::ArrayStart));

    LocationIterator::new(tokenizer)
}
