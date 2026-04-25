#[derive(Debug, Clone)]
enum TokenType {
    // braces
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // operators
    Plus,
    Minus,
    Times,
    Division,

    Number,
    Word,

    // Misc
    EOF,
    Error,
    Whitespace,
}

#[derive(Debug, Clone)]
struct Pos {
    line: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Token {
    value: String,
    token_type: TokenType,
    pos: Pos,
}

fn is_whitespace(value: &str) -> bool {
    return value.chars().all(|x| x.is_whitespace());
}

fn is_numeric(value: &str) -> bool {
    return value.chars().all(|x| x.is_numeric());
}

impl Token {
    fn new(value: &str, line: usize, col: usize) -> Self {
        let mut token = Token {
            value: value.to_string(),
            token_type: TokenType::Word,
            pos: Pos {
                line: line,
                col: col,
            },
        };

        match value {
            "(" => {
                token.token_type = TokenType::LParen;
                token
            }
            ")" => {
                token.token_type = TokenType::RParen;
                token
            }
            "[" => {
                token.token_type = TokenType::LBracket;
                token
            }
            "]" => {
                token.token_type = TokenType::RBracket;
                token
            }
            "{" => {
                token.token_type = TokenType::LBrace;
                token
            }
            "}" => {
                token.token_type = TokenType::RBrace;
                token
            }
            "+" => {
                token.token_type = TokenType::Plus;
                token
            }
            "-" => {
                token.token_type = TokenType::Minus;
                token
            }
            "x" => {
                token.token_type = TokenType::Times;
                token
            }
            "/" => {
                token.token_type = TokenType::Division;
                token
            }
            elemt if is_numeric(elemt) => {
                token.token_type = TokenType::Number;
                token
            }
            elemt if is_whitespace(elemt) => {
                token.token_type = TokenType::Whitespace;
                token
            }
            _ => {
                token.token_type = TokenType::Word;
                token
            }
        }
    }
}

#[derive(Debug, Default)]
struct Tokenizer {
    source: Vec<char>,
    cursor: usize,
    line: usize,
}

impl Tokenizer {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            cursor: 0,
            line: 1,
        }
    }

    fn next_number(&mut self) -> Option<Token> {
        let mut word = String::new();
        while let Some(elmt) = self.source.get(self.cursor)
            && elmt.is_digit(10)
        {
            self.cursor += 1;
            word.push(*elmt);
        }
        Some(Token::new(word.as_str(), self.line, 1))
    }

    fn next_word(&mut self) -> Option<Token> {
        let mut word = String::new();
        while let Some(elmt) = self.source.get(self.cursor)
            && elmt.is_alphanumeric()
        {
            self.cursor += 1;
            word.push(*elmt);
        }
        Some(Token::new(word.as_str(), self.line, 1))
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.source.get(self.cursor)?;
        match value {
            '(' | ')' | '[' | ']' | '{' | '}' | '+' | '-' | 'x' | '/' => {
                self.cursor += 1;
                Some(Token::new(&value.to_string(), self.line, 1))
            }
            value if value.is_whitespace() => {
                self.cursor += 1;
                Some(Token::new(&value.to_string(), self.line, 1))
            }
            value if value.is_digit(10) => self.next_number(),
            _ => self.next_word(),
        }
    }
}

fn main() {
    let lexer = Tokenizer::new("(1 + 2)");
    print!("{:?}", lexer.collect::<Vec<Token>>());
}
