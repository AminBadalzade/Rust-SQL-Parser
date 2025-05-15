use crate::token::{Token, Token::*, Keyword};
use crate::statement::{BinaryOperator, Expression,UnaryOperator};
use crate::sql_parser::Parser;
use std::string::String;
//This function is a shortcut that starts parsing an expression
//In SQL, we can have expressions complex (e.g. with operators, parentheses, etc.).
// This method kicks off the parsing process by calling the main expression parser with the lowest precedence
pub fn parse_expression(parser: &mut Parser) -> Result<Expression, std::string::String> {
    // Start parsing from the lowest priority to handle all operators properly
    parse_binary_expression(parser, 0)
}


pub fn parse_unary_expression(parser: &mut Parser) -> Result<Expression, String> {
    match parser.peek() {
        //To handle unary minus such as -5 or -(-x)
        Token::Minus => {
            parser.advance();

            let expr = parse_unary_expression(parser)?; // recursive for multiple unary ops

            Ok(Expression::UnaryOperation {
                operator: UnaryOperator::Minus,
                operand: Box::new(expr),
            })
        }

        //To handle unary plus such as +5 or ++x
        //Unary plus is generally a no-op, so we just skip it
        Token::Plus => {
            parser.advance();

            //Parsing the next part of the expression normally
            parse_unary_expression(parser)
        }

        // If it's not a unary operator, delegate to primary expression parser
        _ => parse_primary_expression(parser),
    }
}

pub fn parse_primary_expression(parser: &mut Parser) -> Result<Expression, String> {
    match parser.advance() {
        //This will allow us grouping like (a+b) and ensures precedence
        Token::LeftParentheses => {
            let expr = parse_expression(parser)?;
            match parser.advance() {
                Token::RightParentheses => Ok(expr), // If it found closing paren,it returns the grouped expression
                other => Err(format!("Expected ')' after expression, found {:?}", other)), //Error if no closing paren
            }
        }
        Token::Identifier(name) => Ok(Expression::Identifier(name.clone())),
        Token::Number(n) => Ok(Expression::Number(*n)),
        Token::String(s) => Ok(Expression::String(s.clone())),
        Token::Keyword(Keyword::True) => Ok(Expression::Bool(true)),
        Token::Keyword(Keyword::False) => Ok(Expression::Bool(false)),
        other => Err(format!("Unexpected token {:?} - expected primary expression", other)),
    }
}
//This function parses binary expressions using a Pratt parser pattern.
// It handles operator precedence and associativity (e.g., a + b * c is parsed correctly as a + (b * c))
pub fn parse_binary_expression(parser: &mut Parser, min_prec: u8) -> Result<Expression, String> {
    //we start by parsing the left-hand side, which could be a number, identifier, or unary expression
    let mut left = parse_unary_expression(parser)?;

    // Now we handle binary operators in a loop (like +, -, *, etc.)
    while let Some(op) = peek_binary_operator(parser) {
        let prec = get_precedence(&op);
        if prec < min_prec {
            // If the current operator has lower precedence than what we're expecting, stop here
            break;
        }

        parser.advance();

        // Recursively parse the right-hand side with increased precedence
        // This ensures correct grouping like: 1 + 2 * 3 → 1 + (2 * 3)
        let right = parse_binary_expression(parser, prec + 1)?;

        //Finally, we combine left and right expressions into a binary operation
        left = Expression::BinaryOperation {
            left_operand: Box::new(left),
            operator: op,
            right_operand: Box::new(right),
        };
    }

    Ok(left)
}


//It is used to look ahead at the next token and check if it’s a binary operator
//If it is, it returns the corresponding BinaryOperator enum variant
//Some(operator) if the next token is a binary operator (like +, =, AND, etc.)
//None if it's not
pub fn peek_binary_operator(parser: &Parser) -> Option<BinaryOperator> {
    match parser.peek() {
        Equal => Some(BinaryOperator::Equal),
        NotEqual => Some(BinaryOperator::NotEqual),
        GreaterThan => Some(BinaryOperator::GreaterThan),
        GreaterThanOrEqual => Some(BinaryOperator::GreaterThanOrEqual),
        LessThan => Some(BinaryOperator::LessThan),
        LessThanOrEqual => Some(BinaryOperator::LessThanOrEqual),
        Plus => Some(BinaryOperator::Plus),
        Minus => Some(BinaryOperator::Minus),
        Star => Some(BinaryOperator::Multiply),
        Divide => Some(BinaryOperator::Divide),
        Keyword(Keyword::And) => Some(BinaryOperator::And),
        Keyword(Keyword::Or) => Some(BinaryOperator::Or),
        _ => None,
    }
}

//In this program, we will use this function to return the precedence(superiority) of different binary operators in SQL expressions
//In arithmetic and logical operations, precedence determines the order in which operations are applied.
// This function takes a binary operator and returns a number showing how strong it is when used in an expression (its priority level)
pub fn get_precedence(op: &BinaryOperator) -> u8 {
    match op {
        BinaryOperator::Or => 1,
        BinaryOperator::And => 2,
        BinaryOperator::Equal | BinaryOperator::NotEqual => 3,
        BinaryOperator::GreaterThan | BinaryOperator::GreaterThanOrEqual
        | BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual => 4,
        BinaryOperator::Plus | BinaryOperator::Minus => 5,
        BinaryOperator::Multiply | BinaryOperator::Divide => 6,
    }
}