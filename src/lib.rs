#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    // braces
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Period,
    LessThan,
    GreaterThan,
    SemiColon,

    // operators
    Plus,
    Minus,
    Times,
    Division,

    Number,
    Word,

    // Misc
    // EOF,
    // Error,
    Whitespace,
}

#[derive(Debug, Clone, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug, Clone)]
pub struct Token {
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
            pos: Pos(line, col),
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
            ":" => {
                token.token_type = TokenType::Colon;
                token
            }
            "," => {
                token.token_type = TokenType::Comma;
                token
            }
            "." => {
                token.token_type = TokenType::Period;
                token
            }
            "<" => {
                token.token_type = TokenType::LessThan;
                token
            }
            ">" => {
                token.token_type = TokenType::GreaterThan;
                token
            }
            ";" => {
                token.token_type = TokenType::SemiColon;
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type && self.value == other.value && self.pos == other.pos
    }
}

#[derive(Debug, Default)]
pub struct Tokenizer {
    source: Vec<char>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
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
        Some(Token::new(word.as_str(), self.line, self.column))
    }

    fn next_word(&mut self) -> Option<Token> {
        let mut word = String::new();
        while let Some(elmt) = self.source.get(self.cursor)
            && elmt.is_alphanumeric()
        {
            self.cursor += 1;
            word.push(*elmt);
        }
        Some(Token::new(word.as_str(), self.line, self.column))
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.source.get(self.cursor)?;
        match value {
            '(' | ')' | '[' | ']' | '{' | '}' | '+' | '-' | 'x' | '/' | '.' | ':' | '<' | '>'
            | ';' => {
                self.cursor += 1;
                let token = Token::new(&value.to_string(), self.line, self.column);
                self.column += 1;
                return Some(token);
            }
            value if value.is_whitespace() => {
                self.cursor += 1;
                let token = Token::new(&value.to_string(), self.line, self.column);
                self.column += 1;
                if *value == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                return Some(token);
            }
            value if value.is_digit(10) => {
                let token = self.next_number();
                self.column += 1;
                return token;
            }
            _ => {
                let token = self.next_word();
                self.column += 1;
                return token;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let source = "\
  def fibonacci(number):
    pass";
        let lexer = Tokenizer::new(source);
        let expected = vec![
            Token::new("def", 1, 1),
            Token::new(" ", 1, 2),
            Token::new("fibonacci", 1, 3),
            Token::new("(", 1, 4),
            Token::new("number", 1, 5),
            Token::new(")", 1, 6),
            Token::new(":", 1, 7),
            Token::new("\n", 1, 8),
            Token::new(" ", 2, 1),
            Token::new(" ", 2, 2),
            Token::new(" ", 2, 3),
            Token::new(" ", 2, 4),
            Token::new("pass", 2, 5),
        ];
        let lexems = lexer.collect::<Vec<Token>>();
        assert_eq!(lexems, expected);
    }
}
