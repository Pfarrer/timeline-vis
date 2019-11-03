use std::io::Read;

#[derive(PartialEq, Debug)]
pub enum Token {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
}

pub struct JsonTokenizer<T: Read> {
    source: T,
    buf: [u8; 512],
    idx: usize,
    avail: usize,
}

impl<T: Read> JsonTokenizer<T> {
    pub fn new(source: T) -> JsonTokenizer<T> {
        JsonTokenizer {
            source,
            buf: [0; 512],
            idx: 0,
            avail: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespaces();

        match self.get_char() {
            Some(t) => {
                self.idx += 1;
                Some(self.map_token(t))
            }
            None => None,
        }
    }

    fn map_token(&mut self, t: char) -> Token {
        match t {
            '{' => Token::ObjectStart,
            '}' => Token::ObjectEnd,
            '[' => Token::ArrayStart,
            ']' => Token::ArrayEnd,
            '0'..='9' | '-' => self.consume_number(t),
            't' | 'f' => self.consume_bool(t),
            'n' => self.consume_null(t),
            '"' => self.consume_string(t),
            _ => panic!("Unexpected character: {}", t),
        }
    }

    fn consume_whitespaces(&mut self) {
        while self
            .get_char()
            .map_or(false, |c| c.is_whitespace() || c == ',')
        {
            self.idx += 1;
        }
    }

    fn consume_number(&mut self, first: char) -> Token {
        let mut number_str = String::with_capacity(10);
        let mut is_float = false;
        number_str.push(first);

        while let Some(ch) = self.get_char().filter(|c| c.is_digit(10) || *c == '.') {
            if ch == '.' {
                if is_float {
                    panic!("Found multiple '.' in a number");
                } else {
                    is_float = true;
                }
            }
            number_str.push(ch);
            self.idx += 1;
        }

        if is_float {
            Token::Float(number_str.parse().unwrap())
        } else {
            Token::Integer(number_str.parse().unwrap())
        }
    }

    fn consume_bool(&mut self, first: char) -> Token {
        if first == 't' {
            assert_eq!(self.get_char(), Some('r'));
            self.idx += 1;
            assert_eq!(self.get_char(), Some('u'));
            self.idx += 1;
            assert_eq!(self.get_char(), Some('e'));
            self.idx += 1;
            Token::Bool(true)
        } else {
            assert_eq!(self.get_char(), Some('a'));
            self.idx += 1;
            assert_eq!(self.get_char(), Some('l'));
            self.idx += 1;
            assert_eq!(self.get_char(), Some('s'));
            self.idx += 1;
            assert_eq!(self.get_char(), Some('e'));
            self.idx += 1;
            Token::Bool(false)
        }
    }

    fn consume_null(&mut self, _first: char) -> Token {
        assert_eq!(self.get_char(), Some('u'));
        self.idx += 1;
        assert_eq!(self.get_char(), Some('l'));
        self.idx += 1;
        assert_eq!(self.get_char(), Some('l'));
        self.idx += 1;
        Token::Null
    }

    fn consume_string(&mut self, _first: char) -> Token {
        let mut string = String::with_capacity(10);

        while let Some(ch) = self.get_char().filter(|c| *c != '"') {
            string.push(ch);
            self.idx += 1;
        }
        assert_eq!(self.get_char(), Some('"'));
        self.idx += 1;

        self.consume_whitespaces();

        match self.get_char() {
            Some(':') => {
                self.idx += 1;
                Token::Identifier(string)
            }
            _ => Token::String(string),
        }
    }

    fn get_char(&mut self) -> Option<char> {
        if self.idx == self.avail {
            // Buffer consumed, load more data
            self.avail = self.source.read(&mut self.buf).unwrap();
            self.idx = 0;
        }
        if self.avail == 0 {
            // Source completely consumed
            return None;
        }

        let byte = self.buf[self.idx];
        if byte.is_ascii() {
            // Single byte character
            return Some(byte as char);
        } else {
            panic!("Multi-byte UTF8 chars not implemented");
        }
    }
}

impl<T: Read> Iterator for JsonTokenizer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[test]
fn empty_array() {
    let source = "  \r\n [ ] \t   ".as_bytes();
    let mut tokenizer = JsonTokenizer::new(source);

    assert_eq!(tokenizer.next_token(), Some(Token::ArrayStart));
    assert_eq!(tokenizer.next_token(), Some(Token::ArrayEnd));
    assert_eq!(tokenizer.next_token(), None);
}

#[test]
fn array_primitives() {
    let source = "[ \t 123, 13.37, true , \r\nfalse\r\n, null, \"string\"]".as_bytes();
    let mut tokenizer = JsonTokenizer::new(source);

    assert_eq!(tokenizer.next_token(), Some(Token::ArrayStart));
    assert_eq!(tokenizer.next_token(), Some(Token::Integer(123)));
    assert_eq!(tokenizer.next_token(), Some(Token::Float(13.37)));
    assert_eq!(tokenizer.next_token(), Some(Token::Bool(true)));
    assert_eq!(tokenizer.next_token(), Some(Token::Bool(false)));
    assert_eq!(tokenizer.next_token(), Some(Token::Null));
    assert_eq!(tokenizer.next_token(), Some(Token::String("string".into())));
    assert_eq!(tokenizer.next_token(), Some(Token::ArrayEnd));
    assert_eq!(tokenizer.next_token(), None);
}

#[test]
fn numbers() {
    let source = "[ 0,1.0 ,0.1\t,\n9.00003, 0123, -1,-0.01 ]".as_bytes();
    let mut tokenizer = JsonTokenizer::new(source);

    assert_eq!(tokenizer.next_token(), Some(Token::ArrayStart));
    assert_eq!(tokenizer.next_token(), Some(Token::Integer(0)));
    assert_eq!(tokenizer.next_token(), Some(Token::Float(1.0)));
    assert_eq!(tokenizer.next_token(), Some(Token::Float(0.1)));
    assert_eq!(tokenizer.next_token(), Some(Token::Float(9.00003)));
    assert_eq!(tokenizer.next_token(), Some(Token::Integer(123)));
    assert_eq!(tokenizer.next_token(), Some(Token::Integer(-1)));
    assert_eq!(tokenizer.next_token(), Some(Token::Float(-0.01)));
    assert_eq!(tokenizer.next_token(), Some(Token::ArrayEnd));
    assert_eq!(tokenizer.next_token(), None);
}

#[test]
fn empty_object() {
    let source = "  { \r\n\t  }   ".as_bytes();
    let mut tokenizer = JsonTokenizer::new(source);

    assert_eq!(tokenizer.next_token(), Some(Token::ObjectStart));
    assert_eq!(tokenizer.next_token(), Some(Token::ObjectEnd));
    assert_eq!(tokenizer.next_token(), None);
}

#[test]
fn object_primitives() {
    let source = r#"  {"key":"value"  , "num"  : 754,"null":null}   "#.as_bytes();
    let mut tokenizer = JsonTokenizer::new(source);

    assert_eq!(tokenizer.next_token(), Some(Token::ObjectStart));

    assert_eq!(
        tokenizer.next_token(),
        Some(Token::Identifier("key".into()))
    );
    assert_eq!(tokenizer.next_token(), Some(Token::String("value".into())));

    assert_eq!(
        tokenizer.next_token(),
        Some(Token::Identifier("num".into()))
    );
    assert_eq!(tokenizer.next_token(), Some(Token::Integer(754)));

    assert_eq!(
        tokenizer.next_token(),
        Some(Token::Identifier("null".into()))
    );
    assert_eq!(tokenizer.next_token(), Some(Token::Null));

    assert_eq!(tokenizer.next_token(), Some(Token::ObjectEnd));
    assert_eq!(tokenizer.next_token(), None);
}
