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
    Int(i32),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Assign,        // =
    Plus,          // +
    Minus,         // -
    Mult,          // *
    Div,           // /
    Pow,           // **
    GreaterThan,   // >
    LessThan,      // <
    Colon,         // :
    Semicolon,     // ;
    Comma,         // ,
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
