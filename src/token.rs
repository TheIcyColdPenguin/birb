#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Ident(String),
    Symbol(SymbolKind),
    Literal(LiteralKind),
    #[allow(unused)]
    Whitespace,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Assign,
    Plus,
    Semicolon,
    Equals,
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Let,
    If,
    Elif,
    Else,
    Func,
}
