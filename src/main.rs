use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        process::exit(64); // Exit with a usage error code
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                process::exit(65); // Exit with a file-read error code
            });

            let had_error = tokenize(&file_contents);

            if had_error {
                process::exit(65); // Exit with a non-zero error code if tokenization fails
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(64); // Exit with a usage error code
        }
    }
}

fn tokenize(input: &str) -> bool {
    let mut had_error = false;
    let mut chars = input.chars().peekable();
    let mut line = 1;

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

    while let Some(char) = chars.next() {
        match char {
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
                if let Some('/') = chars.peek() {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    println!("SLASH / null");
                }
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(char);
                let mut is_float = false;

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        number.push(next_char);
                        chars.next();
                    } else if next_char == '.' && !is_float {
                        is_float = true;
                        number.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Removed error check for alphanumeric after number

                if is_float {
                    let parsed_number: f64 = number.parse().unwrap();
                    println!("NUMBER {} {:.1}", number, parsed_number);
                } else {
                    println!("NUMBER {} {}.0", number, number);
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(char);

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        identifier.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Some(keyword_type) = keywords.get(identifier.as_str()) {
                    println!("{} {} null", keyword_type, identifier);
                } else {
                    println!("IDENTIFIER {} null", identifier);
                }
            }
            '"' => {
                let mut string_literal = String::new();
                let mut unterminated = true;

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        unterminated = false;
                        println!("STRING \"{}\" {}", string_literal, string_literal);
                        break;
                    } else if c == '\n' {
                        eprintln!("[line {}] Error: Unterminated string.", line);
                        had_error = true;
                        break;
                    } else {
                        string_literal.push(c);
                        chars.next();
                    }
                }

                if unterminated {
                    eprintln!("[line {}] Error: Unterminated string.", line);
                    had_error = true;
                }
            }
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
            '\n' => line += 1,
            ' ' | '\t' | '\r' => {}
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, char);
                had_error = true;
            }
        }
    }

    println!("EOF  null");
    had_error
}

