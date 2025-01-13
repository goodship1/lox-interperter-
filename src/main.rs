use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        process::exit(64); // Exit with a usage error code
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // Read file contents
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                process::exit(65); // Exit with a file-read error code
            });

            // Tokenize the input
            let had_error = tokenize(&file_contents);

            // Exit with an error code if any errors occurred during tokenization
            if had_error {
                process::exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            process::exit(64); // Exit with a usage error code
        }
    }
}

fn tokenize(input: &str) -> bool {
    let mut had_error = false;
    let mut chars = input.chars().peekable(); // Use Peekable for lookahead
    let mut line = 1; // Line tracker

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
                    chars.next(); // Consume the second `/`
                    // Skip the rest of the line for comments
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break; // Stop skipping at the newline
                        }
                        chars.next(); // Consume the comment characters
                    }
                } else {
                    println!("SLASH / null");
                }
            }
            '0'..='9' => {
                // Handle numbers (integers and floats)
                let mut number = String::new();
                number.push(char); // Start with the first digit

                let mut is_float = false;

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit() {
                        number.push(next_char);
                        chars.next(); // Consume the digit
                    } else if next_char == '.' && !is_float {
                        is_float = true; // Mark as float if '.' is found
                        number.push(next_char);
                        chars.next(); // Consume the '.'
                    } else {
                        break;
                    }
                }

                if is_float {
                    // Parse as f64
                    let parsed_number: f64 = number.parse().unwrap();

                    // If the fractional part is zero, print one decimal place (e.g. "87.0");
                    // otherwise print the default float string.
                    if parsed_number.fract() == 0.0 {
                        let normalized = format!("{:.1}", parsed_number);
                        println!("NUMBER {} {}", number, normalized);
                    } else {
                        let normalized = format!("{}", parsed_number);
                        println!("NUMBER {} {}", number, normalized);
                    }
                } else {
                    // Integers remain "NUMBER x x.0"
                    println!("NUMBER {} {}.0", number, number);
                }
            }
            '"' => {
                // Handle string literals
                let mut string_literal = String::new();
                let mut unterminated = true;

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next(); // Consume the closing quote
                        unterminated = false; // String is properly terminated
                        println!("STRING \"{}\" {}", string_literal, string_literal);
                        break;
                    } else if c == '\n' {
                        eprintln!("[line {}] Error: Unterminated string.", line);
                        had_error = true;
                        break;
                    } else {
                        string_literal.push(c);
                        chars.next(); // Consume the character
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
            '\n' => line += 1, // Increment line count for newline characters
            ' ' | '\t' | '\r' => {} // Ignore whitespace
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, char);
                had_error = true;
            }
        }
    }

    println!("EOF  null"); // Always print EOF even if errors occurred
    had_error
}

