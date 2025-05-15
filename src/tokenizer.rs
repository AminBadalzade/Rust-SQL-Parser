use crate::token::{Token, Keyword};

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '(' => { chars.next(); tokens.push(Token::LeftParentheses); }
            ')' => { chars.next(); tokens.push(Token::RightParentheses); }
            ',' => { chars.next(); tokens.push(Token::Comma); }
            ';' => { chars.next(); tokens.push(Token::Semicolon); }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '*' => { chars.next(); tokens.push(Token::Star); }
            '/' => { chars.next(); tokens.push(Token::Divide); }
            '=' => {
                chars.next();
                if matches!(chars.peek(), Some('=')) { chars.next(); } //if we have = after this character,
                tokens.push(Token::Equal); //it thinks like it is ==, adds Token::Equal
            }
            '!' => {
                chars.next();
                if matches!(chars.peek(), Some('=')) { //if we have =, it will return as != (not equal)
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else { //if we have single ! character, it returns an error
                    return Err("Unexpected character '!'".into());
                }
            }
            '>' => {
                chars.next();
                if matches!(chars.peek(), Some('=')) { // we check it is >= or just >
                    chars.next();
                    tokens.push(Token::GreaterThanOrEqual); // if it is >=, it returns as GreaterThanOrEqual
                } else {
                    tokens.push(Token::GreaterThan); //if it is single >, so it is greaterThan
                }
            }
            '<' => {
                chars.next();
                if matches!(chars.peek(), Some('=')) { // we check it is <= or just <
                    chars.next();
                    tokens.push(Token::LessThanOrEqual); // if it is >=, it returns as LessThanOrEqual
                } else {
                    tokens.push(Token::LessThan); //if it is single <, so it is LessThan
                }
            }
            '"' | '\'' => {
                let quote = chars.next().unwrap();
                let mut string = String::new();
                let mut terminated = false;

                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    //it will be something like Some('a').
                    if next_ch == quote { //If the character is the same as the starting quote (e.g. ' or "
                        terminated = true;  //it means the string is finished
                        break; //We set terminated = true and exit the loop.
                    } else {
                        string.push(next_ch); //If it’s not the ending quote, we add the character to the string we’re building.
                    }
                }

                if !terminated { //After the loop: if we didn’t find the closing quote, we return an error
                    return Err(format!("Unterminated string starting with {}{}", quote, string));
                }

                tokens.push(Token::String(string)); //If everything went well, we add the completed string as a token.
            }
            c if c.is_ascii_digit() => {
                let mut num = String::new(); //We create an empty string called num
                while let Some(&c) = chars.peek() { //We keep peeking and reading characters as long as they’re digits
                    if c.is_ascii_digit() {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                //After collecting the digits, it converts the string (e.g. "123") into a number (u64)
                let parsed = num.parse::<u64>().map_err(|_| "Invalid number".to_string())?;
                tokens.push(Token::Number(parsed));
            }

            //This block handles identifiers (e.g., variable names, function names) and keywords (e.g., SELECT, FROM, etc.) in the input.
            //It checks if the current character is alphabetic (a letter) or an underscore (_)
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::new(); //An empty string ident is created to collect the characters that form the identifier
                while let Some(&c) = chars.peek() { //The loop checks the next character and adds it to ident as long as it’s either
                    if c.is_ascii_alphanumeric() || c == '_' { //it can be letter,digit or underscore
                        ident.push(c);
                        chars.next();
                    } else { //If we encounter something that isn’t a letter, digit, or underscore (like a space or punctuation)
                        break; // we are ending loop
                    }
                }
                //There are two options next:
                //For example: we have ident string (select), it converts it to uppercase and checks
                //if it's a keyword using the match_keyword function
                if let Some(keyword) = match_keyword(&ident.to_uppercase()) {
                    tokens.push(Token::Keyword(keyword));
                }
                //If it's not a keyword, it’s treated as a regular identifier (like variable names or table names)
                //So Token::Identifier is added
                else {
                    tokens.push(Token::Identifier(ident));
                }
            }
            //if we have invalid character that don't match none of these patterns
            //This block will handle with this by adding invalid character to the tokens list as a Token::Invalid
            c => {
                chars.next();
                tokens.push(Token::Invalid(c));
            }
        }
    }
    // This part shows that it is end of the input
    //Finally, it returns the list of tokens that were successfully created
    tokens.push(Token::Eof);
    Ok(tokens)
}
//This function takes a string s (a potential keyword) and tries to match it to a known keyword
//If it matches one of the predefined keywords, it returns a Some(Keyword) with the corresponding Keyword enum
// If it doesn't match any keyword,it returns None.
fn match_keyword(s: &str) -> Option<Keyword> {
    match s { //string s to several possible patterns and executes the corresponding block when a match is found
        "SELECT" => Some(Keyword::Select),
        "CREATE" => Some(Keyword::Create),
        "TABLE" => Some(Keyword::Table),
        "WHERE" => Some(Keyword::Where),
        "ORDER" => Some(Keyword::Order),
        "BY" => Some(Keyword::By),
        "ASC" => Some(Keyword::Asc),
        "DESC" => Some(Keyword::Desc),
        "FROM" => Some(Keyword::From),
        "AND" => Some(Keyword::And),
        "OR" => Some(Keyword::Or),
        "NOT" => Some(Keyword::Not),
        "TRUE" => Some(Keyword::True),
        "FALSE" => Some(Keyword::False),
        "PRIMARY" => Some(Keyword::Primary),
        "KEY" => Some(Keyword::Key),
        "CHECK" => Some(Keyword::Check),
        "INT" => Some(Keyword::Int),
        "BOOL" => Some(Keyword::Bool),
        "VARCHAR" => Some(Keyword::Varchar),
        "NULL" => Some(Keyword::Null),
        _ => None,
    }
}

//So, match_keyword takes a string and tries to match it against known keywords.
//if it is one of these keywords (matches), it returns the corresponding Keyword
// If it doesn’t match, it returns None, indicating it wasn’t a recognized keyword