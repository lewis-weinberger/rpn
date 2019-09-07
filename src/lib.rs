use rustyline::Editor;

// Foreground ANSI colour codes
const _BLACK: &str = "\x1b[1;30m";
const _RED: &str = "\x1b[1;31m";
const _GREEN: &str = "\x1b[1;32m";
const _YELLOW: &str = "\x1b[1;33m";
const _BLUE: &str = "\x1b[1;34m";
const _MAGENTA: &str = "\x1b[1;35m";
const _CYAN: &str = "\x1b[1;36m";
const _RESET: &str = "\x1b[0m";

/// Token types.
#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulus,
    EOF,
}

impl Token {
    /// Extract token from string slice
    /// (assuming whitespace has been stripped).
    fn new(slice: &str) -> Token {
        match slice {
            "+" => Token::Add,
            "-" => Token::Subtract,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "^" => Token::Power,
            "%" => Token::Modulus,
            _ => match slice.parse::<f64>() {
                Ok(num) => Token::Number(num),
                Err(_) => Token::EOF,
            },
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        String::from(format!("{:?}", self))
    }
}

/// Functional form of infix addition.
fn add(x: f64, y: f64) -> f64 {
    x + y
}

/// Functional form of infix subtraction.
fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

/// Functional form of infix multiplication.
fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

/// Functional form of infix division.
fn divide(x: f64, y: f64) -> f64 {
    x / y
}

/// Functional form of infix exponentiation.
fn power(x: f64, y: f64) -> f64 {
    x.powf(y)
}

/// Functional form of infix modulus.
fn modulus(x: f64, y: f64) -> f64 {
    x % y
}

/// Iterator over Tokens in String.
struct Tokens {
    /// Buffer holding input split by whitespace.
    buffer: Vec<String>,
    /// Current position within buffer.
    pos: usize,
}

impl Tokens {
    /// Create new Token iterator from String.
    fn new(buffer: String) -> Tokens {
        let split: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();
        Tokens {
            buffer: split,
            pos: 0,
        }
    }
}

impl Iterator for Tokens {
    type Item = Token;

    /// Return next Token.
    fn next(&mut self) -> Option<Self::Item> {
        let next_token = if self.pos < self.buffer.len() {
            Some(Token::new(&self.buffer[self.pos]))
        } else if self.pos == self.buffer.len() {
            Some(Token::EOF)
        } else {
            None
        };

        self.pos += 1;
        next_token
    }
}

/// Interpreter.
pub struct Interpreter {
    tokens: Tokens,
    current: Option<Token>,
    quit: bool,
}

impl Interpreter {
    /// Create new interpreter for user input.
    pub fn new(prompt: &str, editor: &mut Editor<()>) -> rustyline::Result<Interpreter> {
        let coloured_prompt = format!("{}{}{}", _RED, prompt, _RESET);
        let text = editor.readline(&coloured_prompt)?;
        editor.add_history_entry(text.as_str());
        let quit = match text.as_str() {
            "quit" => true,
            "exit" => true,
            _ => false,
        };
        println!("{}===> {}{}", _YELLOW, text, _RESET);
        Ok(Interpreter {
            tokens: Tokens::new(text),
            current: None,
            quit,
        })
    }

    /// Iterate over tokens in user input and evaluate if a valid expression.
    pub fn expr(&mut self) -> Result<bool, String> {
        if self.quit {
            return Ok(true);
        }

        self.current = self.tokens.next();

        let left = match self.current.clone() {
            Some(token) => match token {
                Token::Number(num) => num,
                _ => return Err(token.to_string()),
            },
            None => return Err("unrecognised token".to_string()),
        };

        self.current = self.tokens.next();
        let op = match &self.current {
            Some(token) => match token {
                Token::Add => add,
                Token::Subtract => subtract,
                Token::Multiply => multiply,
                Token::Divide => divide,
                Token::Power => power,
                Token::Modulus => modulus,
                _ => return Err(token.to_string()),
            },
            None => return Err("unrecognised token".to_string()),
        };

        self.current = self.tokens.next();
        let right = match &self.current {
            Some(token) => match token {
                Token::Number(num) => num,
                _ => return Err(token.to_string()),
            },
            None => return Err("unrecognised token".to_string()),
        };

        println!("   = {}", op(left, *right));
        Ok(false)
    }
}
