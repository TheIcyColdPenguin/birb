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
                c if c.is_whitespace() => {
                    while matches!(self.source.peek(), Some(ch) if ch.is_whitespace()) {
                        self.source.next();
                    }
                    self.next_token()
                }
                c if c.is_ascii_alphabetic() || c == '_' => self.parse_alphabetic_token(c),
                c if c.is_ascii_punctuation() => self.parse_punctuation_token(c),
                c => panic!("Unexpected character '{}'", c),
            },
        }
    }

    fn read_while<P>(&mut self, start: char, p: P) -> String
    where
        P: Fn(&char) -> bool + Copy,
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
        let token =
            self.read_while(start, |c| c.is_ascii_alphabetic() || c.is_ascii_digit() || *c == '_');

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
                    SymbolKind::Equals
                }
                _ => SymbolKind::Assign,
            },
            '+' => SymbolKind::Plus,
            '-' => SymbolKind::Minus,
            '*' => SymbolKind::Mult,
            '/' => SymbolKind::Div, // TODO: Add comment functionality
            ';' => SymbolKind::Semicolon,
            '>' => SymbolKind::GreaterThan,
            '<' => SymbolKind::LessThan,

            '{' => SymbolKind::OpenBraces,
            '}' => SymbolKind::CloseBraces,
            '(' => SymbolKind::OpenParens,
            ')' => SymbolKind::CloseParens,
            '[' => SymbolKind::OpenBrackets,
            ']' => SymbolKind::CloseBrackets,

            '\'' => return self.parse_string_literal(start),
            '"' => return self.parse_string_literal(start),
            c => panic!("Unexpected character '{}'", c),
        })
    }

    fn parse_string_literal(&mut self, start: char) -> TokenKind {
        let mut word = String::new();
        let mut escaped = false;
        let mut open = true;

        while let Some(c) = self.source.next() {
            match c {
                '\\' => {
                    if escaped {
                        word.push('\\');
                        escaped = false;
                    } else {
                        escaped = true;
                    }
                }
                c if c == start => {
                    if escaped {
                        word.push(start);
                        escaped = false;
                    } else {
                        open = false;
                        break;
                    }
                }
                c => {
                    if escaped {
                        panic!("TODO: Allow escape characters")
                    } else {
                        word.push(c)
                    }
                }
            }
        }

        if open {
            panic!("Unterminated string")
        }

        TokenKind::Literal(LiteralKind::String(word))
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

        let mut tokenizer = Tokenizer::new("    let _ = x;");
        assert_eq!(tokenizer.next_token(), TokenKind::Keyword(KeywordKind::Let));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("_".into()));
        assert_eq!(tokenizer.next_token(), TokenKind::Symbol(SymbolKind::Assign));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("x".into()));
        assert_eq!(tokenizer.next_token(), TokenKind::Symbol(SymbolKind::Semicolon));
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("    let x = r;");
        assert_eq!(tokenizer.next_token(), TokenKind::Keyword(KeywordKind::Let));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("x".into()));
        assert_eq!(tokenizer.next_token(), TokenKind::Symbol(SymbolKind::Assign));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("r".into()));
        assert_eq!(tokenizer.next_token(), TokenKind::Symbol(SymbolKind::Semicolon));
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("\"hmm\"");
        assert_eq!(tokenizer.next_token(), TokenKind::Literal(LiteralKind::String("hmm".into())));
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);

        let mut tokenizer = Tokenizer::new("let x1 = \"o\";");
        assert_eq!(tokenizer.next_token(), TokenKind::Keyword(KeywordKind::Let));
        assert_eq!(tokenizer.next_token(), TokenKind::Ident("x1".into()));
        assert_eq!(tokenizer.next_token(), TokenKind::Symbol(SymbolKind::Assign));
        assert_eq!(tokenizer.next_token(), TokenKind::Literal(LiteralKind::String("o".into())));
        assert_eq!(tokenizer.next_token(), TokenKind::Symbol(SymbolKind::Semicolon));
        assert_eq!(tokenizer.next_token(), TokenKind::Eof);
    }

    #[test]
    #[should_panic(expected = "Unexpected character '?'")]
    fn it_panics_unexpected_char() {
        let mut tokenizer = Tokenizer::new("\"hmm\" ?");
        assert_eq!(tokenizer.next_token(), TokenKind::Literal(LiteralKind::String("hmm".into())));
        tokenizer.next_token();
    }

    #[test]
    #[should_panic(expected = "Unexpected character 'é'")]
    fn it_panics_unexpected_non_unicode_char() {
        let mut tokenizer = Tokenizer::new("\"hmm\" é");
        assert_eq!(tokenizer.next_token(), TokenKind::Literal(LiteralKind::String("hmm".into())));
        tokenizer.next_token();
    }

    #[test]
    #[should_panic(expected = "TODO: Allow escape characters")]
    fn it_panics_escape_char_in_string() {
        let mut tokenizer = Tokenizer::new(r#""hmm\n ok""#);
        tokenizer.next_token();
    }

    #[test]
    #[should_panic(expected = "Unterminated string")]
    fn it_panics_unexpected_eof() {
        let mut tokenizer = Tokenizer::new(r#""hmm"#);
        assert_eq!(tokenizer.next_token(), TokenKind::Literal(LiteralKind::String("hmm".into())));
    }

    #[test]
    #[should_panic(expected = "Unterminated string")]
    fn it_panics_unexpected_eof_due_to_escaping() {
        let mut tokenizer = Tokenizer::new(r#""hmm\"#);
        assert_eq!(tokenizer.next_token(), TokenKind::Literal(LiteralKind::String("hmm".into())));
    }
}
