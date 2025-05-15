use std::string::String;
use crate::token::{Token, Token::*, Keyword};
use crate::statement::{UnaryOperator, *};
use crate::pratt_parsing::parse_expression;

// This struct holds the list of tokens and keeps track of the current position
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// In this block, we will create a new parser from a list of tokens
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // The parse() method looks at the first token (peek()) and decides which kind of SQL statement to parse (SELECT or CREATE)
    // If it is Select keyword, then we will parse_select() method
    // In other case, we will call parse_create() method
    pub fn parse(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Keyword(Keyword::Select) => self.parse_select(),
            Token::Keyword(Keyword::Create) => self.parse_create(),
            // If it's neither, it returns an error
            _ => Err("Expected SELECT or CREATE statement".to_string()),
        }
    }

    // It expects the keyword SELECT to appear first. If it's not there, it will return an error and stop
    fn parse_select(&mut self) -> Result<Statement, String> {
        self.expect_keyword_any_line(Keyword::Select)?;
        // This creates an empty list called columns to store the columns selected in the query (like SELECT name, age)
        let mut columns = Vec::new();

        // Keep collecting column expressions until we see FROM
        loop {
            match self.peek() {
                Token::Star => {
                    // Here, we are handling with SELECT *
                    self.advance();
                    columns.push(Expression::AllColumns); // Make sure you add this variant in your Expression enum
                }
                Token::Keyword(Keyword::From) => {
                    // If FROM appears and no columns collected, it's an error
                    if columns.is_empty() {
                        return Err("Expected at least one column before FROM".to_string());
                    }
                    break; // End of column list
                }
                _ => {
                    // if we don't encounter with * star, then we will return as a normal expression
                    // we will wait for column name or expression
                    columns.push(parse_expression(self)?);
                }
            }

            // We are expecting comma or FROM after each column
            match self.peek() {
                Comma => {
                    self.advance();
                    // After a comma, ensure the next token is not FROM (no trailing comma allowed)
                    if self.match_keyword(Keyword::From) {
                        return Err("Trailing comma before FROM is not allowed".to_string());
                    }
                }
                Token::Keyword(Keyword::From) => break, // if it is From, we will think it as end of column
                _ => return Err(format!("Expected ',' or FROM, found {:?}", self.peek())),
            }
        }

        // Parsing FROM clause (table name)
        self.expect_keyword_any_line(Keyword::From)?;
        let from = match self.advance() {
            Identifier(name) => name.clone(),
            _ => return Err("Expected table name after FROM".to_string()),
        };

        // We can have WHERE keyword also:
        let r#where = if self.match_keyword(Keyword::Where) {
            self.advance();
            Some(parse_expression(self)?)
        } else {
            None
        };

        // ORDER BY:
        let mut orderby = Vec::new();
        if self.match_keyword(Keyword::Order) {
            self.advance();
            // After ORDER keyword,we expect the next keyword to be "BY", return an error
            self.expect_keyword_any_line(Keyword::By)?;
            loop {
                let mut expr = parse_expression(self)?;

                // Check if the next token is a sorting direction keyword: ASC or DESC
                match self.peek() {
                    // If "ASC", advance the token stream and wrap the expression with a unary operation for ascending order
                    Keyword(Keyword::Asc) => {
                        self.advance();
                        expr = Expression::UnaryOperation {
                            operand: Box::new(expr),
                            operator: UnaryOperator::Asc,
                        };
                    }
                    // If "DESC", advance the token stream and wrap the expression similarly for descending order
                    Keyword(Keyword::Desc) => {
                        self.advance();
                        expr = Expression::UnaryOperation {
                            operand: Box::new(expr),
                            operator: UnaryOperator::Desc,
                        };
                    }
                    // Otherwise, no explicit order direction is specified (default order assumed)
                    _ => {}
                }
                // Add the parsed expression (with optional order operator) to the orderby list
                orderby.push(expr);

                // If the next token is NOT a comma, we will break out of the loop (end of ORDER BY clause)
                if !self.match_token(Comma) { break; }

                // If there is a comma,it will advance to the next token to parse the next expression
                self.advance();
            }
        }
        // This line is calling the expect_semicolon() method, which checks if the next token is a semicolon (;)
        // If we miss the semicolon at the end, it will return error in return
        self.expect_semicolon()?;

        // Return the parsed SELECT statement, including the columns, FROM clause, optional WHERE clause,
        // and the ORDER BY expressions collected above
        Ok(Statement::Select { columns, from, r#where, orderby })
    }

    fn parse_create(&mut self) -> Result<Statement, String> {
        // We start by expecting the CREATE keyword and then the TABLE keyword.
        // The expect_keyword() method checks if the current token matches the expected keyword
        self.expect_keyword_any_line(Keyword::Create)?;
        self.expect_keyword_any_line(Keyword::Table)?;

        // After CREATE TABLE, the next token should be the table name
        let table_name = match self.advance() {
            // If that token is an Identifier, it gets the table name (which is a string) and clones it
            Identifier(name) => name.clone(),
            // If the next token is not an Identifier (i.e., not a valid table name),
            // we return an error saying "Expected table name after CREATE TABLE."
            _ => return Err("Expected table name after CREATE TABLE".to_string()),
        };

        // After the table name, we expect an opening parenthesis ( to start the list of column definitions
        self.expect_token_any_line(Token::LeftParentheses)?;

        // We enter a loop to parse each column definition- The column name is stored in the column_name variable
        let mut column_list = Vec::new();
        loop {
            // If we see a closing parenthesis, it means we've reached the end of the column list
            if self.match_token(Token::RightParentheses) {
                break; // do not consume here, handled below
            }

            // The next token should be a column name (an identifier)
            let column_name = match self.advance() {
                Identifier(name) => name.clone(),
                _ => return Err("Expected column name".to_string()),
            };

            // Then parse the column type, e.g., INT, BOOL, or VARCHAR with a length
            let column_type = match self.advance() {
                Keyword(Keyword::Int) => DBType::Int,
                Keyword(Keyword::Bool) => DBType::Bool,
                Keyword(Keyword::Varchar) => {
                    // For VARCHAR, allow parentheses with a length number inside or default length
                    if self.match_token(Token::LeftParentheses) {
                        self.expect_token_any_line(Token::LeftParentheses)?;
                        let len = match self.advance() {
                            Number(n) => *n as usize,
                            _ => return Err("Expected number in VARCHAR(n)".to_string()),
                        };
                        self.expect_token_any_line(Token::RightParentheses)?;
                        DBType::Varchar(len)
                    } else {
                        DBType::Varchar(255) // default length if unspecified
                    }
                }
                // If the token is not a valid column type, we return an error saying "Expected column type"
                _ => return Err("Expected column type (INT, BOOL, VARCHAR)".to_string()),
            };

            // After parsing the column type, we check if there are any constraints associated with
            // the column, like NOT NULL, PRIMARY KEY, or CHECK.
            let mut constraints = Vec::new();
            loop {
                match self.peek() {
                    Keyword(Keyword::Not) => {
                        // If we encounter the NOT NULL constraint, we add Constraint::NotNull to the list
                        self.advance();
                        self.expect_keyword_any_line(Keyword::Null)?;
                        constraints.push(Constraint::NotNull);
                    }
                    Keyword(Keyword::Primary) => {
                        /// If we encounter PRIMARY KEY, we add Constraint::PrimaryKey to the list
                        self.advance();
                        self.expect_keyword_any_line(Keyword::Key)?;
                        constraints.push(Constraint::PrimaryKey);
                    }
                    Keyword(Keyword::Check) => {
                        // If we encounter a CHECK constraint, we parse an expression for the check condition and add Constraint::Check to the list
                        self.advance();
                        self.expect_token_any_line(Token::LeftParentheses)?;
                        let expr = parse_expression(self)?;
                        self.expect_token_any_line(Token::RightParentheses)?;
                        constraints.push(Constraint::Check(expr));
                    }
                    _ => break, // If no constraints are found, we break out of the loop
                }
            }

            // After parsing the column name, type, and constraints, we create a TableColumn and add it to the column_list
            column_list.push(TableColumn { column_name, column_type, constraints });

            // After each column definition, we expect either a comma (,) to separate columns or a closing parenthesis ())
            // If we encounter something else, we return an error saying that we expected either a comma or a closing parenthesis.
            match self.peek() {
                Comma => { self.advance(); },
                Token::RightParentheses => {
                    self.advance(); // consume ')'
                    break;
                },
                _ => return Err("Expected ',' or ')' in column definition list".to_string()),
            }
        }
        // After finishing the column definitions, we expect the SQL statement to end with a semicolon (;)
        self.expect_semicolon()?;
        // If everything goes correctly, it returns a CreateTable statement
        Ok(Statement::CreateTable { table_name, column_list })
    }


    // The expect_token_any_line function checks if the next token matches the expected token type,
    // regardless of whether the formatting includes newlines or spaces between tokens
    //expected: The token we’re expecting (e.g., LeftParentheses, Comma, Identifier, etc.)
    fn expect_token_any_line(&mut self, expected: Token) -> Result<(), String> {
        if self.match_token(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected token {:?}, got {:?}", expected, self.peek()))
        }
    }

    // The expect_keyword_any_line function checks if the next token is the expected keyword,
    // regardless of whether it's on a new line or the same line.
    fn expect_keyword_any_line(&mut self, kw: Keyword) -> Result<(), String> {
        if self.match_keyword(kw.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected keyword {:?}, got {:?}", kw, self.peek()))
        }
    }

    //Here it checks if the next token is a semicolon; advances if yes, otherwise returns an error.
    fn expect_semicolon(&mut self) -> Result<(), String> {
        match self.peek() {
            Semicolon => {
                self.advance();
                Ok(())
            }
            _ => Err(format!("Expected semicolon, found {:?}", self.peek())),
        }
    }

    ///It returns true if the next token is the keyword we are looking for, otherwise false
    fn match_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.peek(), Keyword(k) if *k == keyword)
    }

    // Checks if the next token is of the same type as the expected token
    // Useful for matching token categories like any Identifier or Keyword.
    fn match_token(&self, expected: Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected)
    }

    // Returns a reference to the current token without advancing the parser.
    // If we are at the end of the token stream, returns an End-Of-File (Eof) token as a sentinel
    pub(crate) fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Eof)
    }

    //This function advances the parser to the next token and returns it
    pub(crate) fn advance(&mut self) -> &Token {
        let idx = self.current;
        self.current += 1;
        self.tokens.get(idx).unwrap_or(&Eof)
    }
}