use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // We expect something like: <program> tokenize <filename>
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        process::exit(64); // Usage error
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // Attempt to read file
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                process::exit(65); // File read error
            });

            // Tokenize file contents
            let had_error = tokenize(&file_contents);

            // If any scanning errors occurred, exit code 65
            if had_error {
                process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(64); // Usage error
        }
    }
}

/// Scans the input and prints tokens. Returns true if any scanning errors occurred.
fn tokenize(input: &str) -> bool {
    let mut had_error = false;
    let mut chars = input.chars().peekable();
    let mut line = 1;

    // Keywords in Lox
    let keywords: HashMap<&str, &str> = [
        ("and", "AND"),
        ("class", "CLASS"),
        ("else", "ELSE"),
        ("false", "FALSE"),
        ("for", "FOR"),
        ("fun", "FUN"),
        ("if", "IF"),
        ("nil", "NIL"),
        ("or", "OR"),
        ("print", "PRINT"),
        ("return", "RETURN"),
        ("super", "SUPER"),
        ("this", "THIS"),
        ("true", "TRUE"),
        ("var", "VAR"),
        ("while", "WHILE"),
    ]
    .iter()
    .cloned()
    .collect();

    while let Some(ch) = chars.next() {
        match ch {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            '.' => println!("DOT . null"),
            '+' => println!("PLUS + null"),
            ',' => println!("COMMA , null"),
            '-' => println!("MINUS - null"),
            ';' => println!("SEMICOLON ; null"),
            '/' => {
                // Look ahead for comment
                if let Some('/') = chars.peek() {
                    // This is a comment; consume '//'
                    chars.next();
                    // Skip until newline
                    while let Some(&comment_char) = chars.peek() {
                        if comment_char == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    println!("SLASH / null");
                }
            }

            // Number literal (integer or float)
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);
                let mut is_float = false;

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        number.push(next_ch);
                        chars.next();
                    } else if next_ch == '.' && !is_float {
                        is_float = true;
                        number.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if is_float {
                    let parsed = number.parse::<f64>().unwrap();
                    println!("NUMBER {} {}", number, format_float_value(parsed));
                } else {
                    // No decimal point => integer
                    println!("NUMBER {} {}.0", number, number);
                }
            }

            // Identifiers or keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(ch);

                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        identifier.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check if it matches a known keyword
                if let Some(token_type) = keywords.get(identifier.as_str()) {
                    println!("{} {} null", token_type, identifier);
                } else {
                    println!("IDENTIFIER {} null", identifier);
                }
            }

            // String literal
            '"' => {
                let mut string_literal = String::new();
                let mut unterminated = true;

                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '"' {
                        // Closing quote
                        chars.next(); // consume it
                        unterminated = false;
                        println!("STRING \"{}\" {}", string_literal, string_literal);
                        break;
                    } else if next_ch == '\n' {
                        eprintln!("[line {}] Error: Unterminated string.", line);
                        had_error = true;
                        break;
                    } else {
                        string_literal.push(next_ch);
                        chars.next();
                    }
                }

                if unterminated {
                    // We never found a closing quote
                    eprintln!("[line {}] Error: Unterminated string.", line);
                    had_error = true;
                }
            }

            // Comparison operators
            '<' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    println!("LESS_EQUAL <= null");
                } else {
                    println!("LESS < null");
                }
            }
            '>' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    println!("GREATER_EQUAL >= null");
                } else {
                    println!("GREATER > null");
                }
            }
            '!' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    println!("BANG_EQUAL != null");
                } else {
                    println!("BANG ! null");
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    println!("EQUAL_EQUAL == null");
                } else {
                    println!("EQUAL = null");
                }
            }

            // Newline
            '\n' => line += 1,

            // Whitespace
            ' ' | '\t' | '\r' => {},

            // Unknown character => error
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, ch);
                had_error = true;
            }
        }
    }

    // End of file
    println!("EOF  null");

    had_error
}

/// Ensures floats have at least one digit after the decimal if there's no fractional part.
fn format_float_value(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}.0", value.trunc())
    } else {
        value.to_string() // minimal decimal representation
    }
}
