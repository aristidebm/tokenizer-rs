#[derive(Debug, Clone)]
enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Whitespace,
    EOF,
}

#[derive(Debug, Clone)]
struct Pos {
    line: u32,
    col: u32,
}

#[derive(Debug, Clone)]
struct Token {
    value: String,
    token_type: TokenType,
    pos: Pos,
}

#[derive(Debug, Default)]
struct Tokenizer {
    source: String,
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = Token {
            value: "(".to_string(),
            token_type: TokenType::LParen,
            pos: Pos { line: 1, col: 1 },
        };
        Some(token)
    }
}

fn main() {
    let mut lexer = Tokenizer {
        source: "".to_string(),
    };
    println!("{:?}", lexer.next().unwrap());
}
