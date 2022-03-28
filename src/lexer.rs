use crate::token::*;

use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    pub source: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new<S>(source: S) -> Tokenizer<'a>
    where
        S: Into<&'a str>,
    {
        Tokenizer {
            source: source.into().chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> TokenKind {
        match self.source.next() {
            None => TokenKind::Eof,
            Some(chr) => match chr {
                _ if chr.is_whitespace() => {
                    while matches!(self.source.peek(), Some(c) if c.is_whitespace()) {
                        self.source.next();
                    }
                    self.next_token()
                }
                c if chr.is_ascii_alphabetic() => self.parse_alphabetic_token(c),
                c if chr.is_ascii_punctuation() => self.parse_punctuation_token(c),
                c => panic!("Unexpected character '{}'", c),
            },
        }
    }

    fn read_while<P>(&mut self, start: char, p: P) -> String
    where
        P: FnOnce(&char) -> bool + Copy,
    {
        let mut word = String::from(start);
        while matches!(self.source.peek(), Some(c) if p(c)) {
            match self.source.next() {
                Some(c) => word.push(c),
                None => unreachable!(),
            }
        }

        word
    }

    fn parse_alphabetic_token(&mut self, start: char) -> TokenKind {
        let token = self.read_while(start, |c| c.is_ascii_alphabetic() || *c == '_');

        match token.as_str() {
            "let" => TokenKind::Keyword(KeywordKind::Let),
            "if" => TokenKind::Keyword(KeywordKind::If),
            "elif" => TokenKind::Keyword(KeywordKind::Elif),
            "else" => TokenKind::Keyword(KeywordKind::Else),
            "func" => TokenKind::Keyword(KeywordKind::Func),
            _ => TokenKind::Ident(token),
        }
    }

    fn parse_punctuation_token(&mut self, start: char) -> TokenKind {
        TokenKind::Symbol(match start {
            '=' => match self.source.peek() {
                Some('=') => {
                    self.source.next();
                    SymbolKind::Equality
                }
                _ => SymbolKind::Assign,
            },
            ';' => SymbolKind::Semicolon,
            c => panic!("Unexpected character '{}'", c),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_next_token() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("    ");
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("    let");
        assert_eq!(tokenizer.next_token(), TokenKind::Keyword(KeywordKind::Let));
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("    let x");
        assert_eq!(tokenizer.next_token(), TokenKind::Keyword(KeywordKind::Let));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("x".into()));
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("    let x = r;");
        assert_eq!(tokenizer.next_token(), TokenKind::Keyword(KeywordKind::Let));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("x".into()));
        assert_eq!(
            tokenizer.next_token(),
            TokenKind::Symbol(SymbolKind::Assign)
        );
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("r".into()));
        assert_eq!(
            tokenizer.next_token(),
            TokenKind::Symbol(SymbolKind::Semicolon)
        );
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);
    }
}
