use std::io::{self, Write};
use crate::tokenizer::tokenize;
use crate::sql_parser::Parser;
use crate::statement::Statement;

mod token;
mod tokenizer;
mod sql_parser;
mod statement;
mod pratt_parsing;

mod ForBonusPoints;

fn main() {
    println!("Please, enter your SQL queries to check my SQL Parser");
    println!("If you want to exit the program, just type \"exit\" or \"quit\"");

    // This buffer accumulates user input across multiple lines until a semicolon is reached
    let mut buffer = String::new(); // Accumulate multi-line input until semicolon

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Read a line of user input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        // Check if the user wants to exit
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            println!("Exiting.");
            break;
        }

        // Add the user's input to the buffer and make sure there's a space after it
        buffer.push_str(input.trim_end());
        buffer.push(' ');

        // Only parse when we detect the end of a full SQL statement marked by a semicolon
        if buffer.trim_end().ends_with(';') {
            match tokenize(&buffer) {
                Ok(tokens) => {
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(statement) => {
                            // Pretty-print the successfully parsed SQL AST
                            println!("PARSED SUCCESFULLY, here is:\n{:#?}", statement);
                        }
                        Err(err) => {
                            println!("Parser error: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Tokenizer error: {}", err);
                }
            }
            // Clear the buffer to be ready for the next statement
            buffer.clear();
        }
    }
}
