use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <tokenize|parse> <filename>", args[0]);
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

        "parse" => {
            // Same file read
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                process::exit(65); // File read error
            });

            // Parse returns true if there's an error
            let had_error = parse(&file_contents);

            // If any parse errors occurred, exit code 65
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

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        identifier.push(next_char);
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

/// ---------------------------------------------------------------------------
/// PARSER
/// ---------------------------------------------------------------------------

/// Minimal token type used for parsing.
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    // Single chars
    LeftParen, RightParen,
    // Literals
    Number(f64),
    StringLit(String),
    True, False, Nil,
    // We won't parse all the tokens above for this mini stage, just enough
    // to show booleans, nil, numbers, parentheses, and strings.
    Eof,
}

/// A Token for the parser
#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,  // the exact text
    line: usize,
}

/// A minimal expression AST for demonstration.
#[derive(Debug, Clone)]
enum Expr {
    Literal(LitValue),
    Grouping(Box<Expr>),
}

/// Literal values we care about
#[derive(Debug, Clone)]
enum LitValue {
    Boolean(bool),
    Nil,
    Number(f64),
    Str(String),
}

/// The parser itself
struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

   
    fn parse(&mut self) -> Option<Expr> {
        let expr = self.expression();

        if self.had_error {
            None
        } else {
            Some(expr)
        }
    }

    /
    fn expression(&mut self) -> Expr {
        self.primary()
    }

    /// primary -> "true" | "false" | "nil" | NUMBER | STRING | "(" expression ")"
    fn primary(&mut self) -> Expr {
        // Peek current token
        let token = self.peek_token();

        match token.token_type {
            TokenType::True => {
                self.advance();
                Expr::Literal(LitValue::Boolean(true))
            }
            TokenType::False => {
                self.advance();
                Expr::Literal(LitValue::Boolean(false))
            }
            TokenType::Nil => {
                self.advance();
                Expr::Literal(LitValue::Nil)
            }
            TokenType::Number(n) => {
                self.advance();
                Expr::Literal(LitValue::Number(n))
            }
            TokenType::StringLit(ref s) => {
                // clone s
                let lit_string = s.clone();
                self.advance();
                Expr::Literal(LitValue::Str(lit_string))
            }
            TokenType::LeftParen => {
                self.advance(); // consume '('
                let expr = self.expression();
                // Expect a right paren
                if self.peek_token().token_type == TokenType::RightParen {
                    self.advance(); // consume it
                } else {
                    self.error("Expected ')' after expression.");
                }
                Expr::Grouping(Box::new(expr))
            }
            TokenType::RightParen | TokenType::Eof => {
                // Error: we expected an expression but got a right paren or end
                self.error("Expected expression.");
                // Return something to keep going
                Expr::Literal(LitValue::Nil)
            }
        }
    }

    /// If there's an error, print message and set had_error.
    fn error(&mut self, msg: &str) {
        eprintln!("Parse error: {}", msg);
        self.had_error = true;
    }

    /// Return the current token
    fn peek_token(&self) -> &Token {
        if self.current >= self.tokens.len() {
            // In case we're out of range
            &self.tokens[self.tokens.len() - 1]
        } else {
            &self.tokens[self.current]
        }
    }

    /// Advance the parser by one token
    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }
}

/// A small "AST printer" that prints expressions in a Lisp-like style:
/// - `true`, `false`, `nil` for booleans/nil
/// - e.g. `123.0` for numbers
/// - e.g. `("hello")` for strings
/// - `(group <expr>)` or just `(<expr>)` if you prefer
fn print_ast(expr: &Expr) -> String {
    match expr {
        Expr::Literal(value) => match value {
            LitValue::Boolean(b) => b.to_string(), // "true" or "false"
            LitValue::Nil => "nil".to_string(),
            LitValue::Number(n) => format_float_value(*n), // e.g. "3.0"
            LitValue::Str(s) => format!("\"{}\"", s),
        },
        Expr::Grouping(sub) => format!("(group {})", print_ast(sub)),
    }
}

/// ---------------------------------------------------------------------------
/// parse() function: Scans => converts to simpler tokens => runs parser => prints AST
/// Returns `true` on error, `false` if success.
/// ---------------------------------------------------------------------------
fn parse(source: &str) -> bool {
    // 1) Scan to "raw" tokens with your existing scanning logic,
    //    but we won't print them. We'll convert them into the `TokenType`
    //    used by the parser (`TokenType::True, TokenType::Nil`, etc.)

    let raw_tokens = scan_raw_tokens(source);

    // 2) Build the parser tokens
    let mut parser_tokens = Vec::new();
    for rtok in raw_tokens {
        parser_tokens.push(convert_token(rtok));
    }

    // 3) Parse
    let mut parser = Parser::new(parser_tokens);
    let ast = parser.parse();

    // 4) If parse error, return true
    if parser.had_error || ast.is_none() {
        return true;
    }

    // 5) Otherwise, print the AST
    let expr = ast.unwrap();
    println!("{}", print_ast(&expr));
    false
}

/// A minimal "raw" token representation from your existing scanner output.
/// We only store the type as a string for now, plus the lexeme and line.
#[derive(Debug)]
struct RawToken {
    token_type: String,
    lexeme: String,
    line: usize,
}

/// We'll do a custom scanning that returns `Vec<RawToken>` instead of printing.
/// This is a simplified version of `tokenize()`, but just collects tokens.
fn scan_raw_tokens(source: &str) -> Vec<RawToken> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;

    while let Some(ch) = chars.next() {
        match ch {
            '(' => {
                tokens.push(RawToken {
                    token_type: "LEFT_PAREN".into(),
                    lexeme: "(".into(),
                    line,
                });
            }
            ')' => {
                tokens.push(RawToken {
                    token_type: "RIGHT_PAREN".into(),
                    lexeme: ")".into(),
                    line,
                });
            }
            '"' => {
                // String
                let mut string_literal = String::new();
                let mut unterminated = true;

                while let Some(&nc) = chars.peek() {
                    if nc == '"' {
                        chars.next(); // consume closing "
                        unterminated = false;
                        break;
                    } else if nc == '\n' {
                        line += 1;
                    }
                    string_literal.push(nc);
                    chars.next();
                }

                if unterminated {
                    // We didn't find a closing quote
                    // We'll still record it, but note it might be invalid
                }
                tokens.push(RawToken {
                    token_type: "STRING".into(),
                    lexeme: string_literal,
                    line,
                });
            }
            '0'..='9' => {
                // number
                let mut number_str = ch.to_string();
                let mut is_float = false;
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() {
                        number_str.push(nc);
                        chars.next();
                    } else if nc == '.' && !is_float {
                        is_float = true;
                        number_str.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(RawToken {
                    token_type: "NUMBER".into(),
                    lexeme: number_str,
                    line,
                });
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // identifier or keyword
                let mut ident = ch.to_string();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' {
                        ident.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(RawToken {
                    token_type: identify_keyword(&ident),
                    lexeme: ident,
                    line,
                });
            }
            '\n' => {
                line += 1;
            }
            ' ' | '\r' | '\t' => {
                // ignore
            }
            _ => {
                // ignore or handle error
            }
        }
    }

    // push an EOF
    tokens.push(RawToken {
        token_type: "EOF".into(),
        lexeme: "".into(),
        line,
    });

    tokens
}

/// Determine if the given identifier is 'true', 'false', or 'nil'.
fn identify_keyword(s: &str) -> String {
    match s {
        "true" => "TRUE".into(),
        "false" => "FALSE".into(),
        "nil" => "NIL".into(),
        _ => "IDENTIFIER".into(),
    }
}

/// Converts the "raw" token (which only has a string type) into a parser `Token`.
fn convert_token(rtok: RawToken) -> Token {
    use TokenType::*;
    let token_type = match rtok.token_type.as_str() {
        "LEFT_PAREN" => LeftParen,
        "RIGHT_PAREN" => RightParen,
        "STRING" => StringLit(rtok.lexeme.clone()),
        "TRUE" => True,
        "FALSE" => False,
        "NIL" => Nil,
        "NUMBER" => {
            // parse float
            let val = rtok.lexeme.parse::<f64>().unwrap_or(0.0);
            Number(val)
        }
        "EOF" => Eof,
        _ => {
           
            Nil
        }
    };

    Token {
        token_type,
        lexeme: rtok.lexeme,
        line: rtok.line,
    }
}
