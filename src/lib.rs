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
    Invalid,
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
                Err(_) => Token::Invalid,
            },
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        String::from(format!("{:?}", self))
    }
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
        } else {
            None
        };

        self.pos += 1;
        next_token
    }
}

/// Interpreter.
pub struct Interpreter {
    buffer: Vec<Token>,
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
        let buffer: Vec<Token> = Tokens::new(text).collect(); 
        Ok(Interpreter { buffer, quit })
    }

    /// Evaluate stack.
    pub fn expr(&self) -> Result<bool, String> {
        if self.quit {
            return Ok(true);
        }

        let mut stack: Vec<f64> = Vec::new();

        for token in &self.buffer {
            match token {
                Token::Number(num) => stack.push(*num),
                Token::Add => {
                    let (x, y) = stack.binary_pop()?;
                    stack.push(x + y)
                },
                Token::Subtract => {
                    let (x, y) = stack.binary_pop()?;
                    stack.push(x - y)
                },
                Token::Multiply => {
                    let (x, y) = stack.binary_pop()?;
                    stack.push(x * y)
                },
                Token::Divide => {
                    let (x, y) = stack.binary_pop()?;
                    stack.push(x / y)
                },
                Token::Power => {
                    let (x, y) = stack.binary_pop()?;
                    stack.push(x.powf(y))
                },
                Token::Modulus => {
                    let (x, y) = stack.binary_pop()?;
                    stack.push(x % y)
                },
                Token::Invalid => return Err("invalid input".to_string()),
            }
        }

        if stack.len() == 1 {
            match stack.pop() {
                Some(num) => println!("   = {}", num),
                None => return Err("invalid input".to_string())
            }
        } else {
            return Err("invalid input".to_string())
        }
        
        Ok(false)
    }
}

trait DoublePop {
    fn binary_pop(&mut self) -> Result<(f64, f64), String>;
}

impl DoublePop for Vec<f64> {
    fn binary_pop(&mut self) -> Result<(f64, f64), String> {
        let x = match self.pop() {
            Some(num) => num,
            None => return Err("invalid number.".to_string())
        };
        let y = match self.pop() {
            Some(num) => num,
            None => return Err("invalid number".to_string())
        };
        Ok((x, y))
    }
}
