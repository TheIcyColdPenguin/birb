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
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Assign,        // =
    Plus,          // +
    Minus,         // -
    Mult,          // *
    Div,           // /
    GreaterThan,   // >
    LessThan,      // <
    Semicolon,     // ;
    Equals,        // ==
    OpenBraces,    // {
    CloseBraces,   // }
    OpenParens,    // (
    CloseParens,   // )
    OpenBrackets,  // [
    CloseBrackets, // ]
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Let,
    If,
    Elif,
    Else,
    Func,
}
