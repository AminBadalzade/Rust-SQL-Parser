use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(u64),
    Invalid(char),
    RightParentheses,
    LeftParentheses,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Star,
    Divide,
    Minus,
    Plus,
    Comma,
    Semicolon,
    Eof,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Keyword {
    Select,
    Create,
    Table,
    Where,
    Order,
    By,
    Asc,
    Desc,
    From,
    And,
    Or,
    Not,
    True,
    False,
    Primary,
    Key,
    Check,
    Int,
    Bool,
    Varchar,
    Null,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Identifier(iden) => write!(f, "{:?}", iden),
            Token::String(str) => write!(f, "{:?}", str),
            Token::Number(num) => write!(f, "{:?}", num),
            Token::RightParentheses => write!(f, ")"),
            Token::LeftParentheses => write!(f, "("),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanOrEqual => write!(f, ">="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanOrEqual => write!(f, "<="),
            Token::Equal => write!(f, "="),
            Token::NotEqual => write!(f, "!="),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Eof => write!(f, "Eof"),
            Token::Invalid(c) => write!(f, "Invalid({})", c),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Select => write!(f, "SELECT"),
            Keyword::Create => write!(f, "CREATE"),
            Keyword::Table => write!(f, "TABLE"),
            Keyword::Where => write!(f, "WHERE"),
            Keyword::Order => write!(f, "ORDER"),
            Keyword::By => write!(f, "BY"),
            Keyword::Asc => write!(f, "ASC"),
            Keyword::Desc => write!(f, "DESC"),
            Keyword::From => write!(f, "FROM"),
            Keyword::And => write!(f, "AND"),
            Keyword::Or => write!(f, "OR"),
            Keyword::Not => write!(f, "NOT"),
            Keyword::True => write!(f, "TRUE"),
            Keyword::False => write!(f, "FALSE"),
            Keyword::Primary => write!(f, "PRIMARY"),
            Keyword::Key => write!(f, "KEY"),
            Keyword::Check => write!(f, "CHECK"),
            Keyword::Int => write!(f, "INT"),
            Keyword::Bool => write!(f, "BOOL"),
            Keyword::Varchar => write!(f, "VARCHAR"),
            Keyword::Null => write!(f, "NULL"),
        }
    }
}
