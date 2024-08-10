struct Lexer<'a> {
    input: &'a String,
    position: usize,
    read_position: usize,
    ch: char,
    current_identifier: str,
}

#[derive(Debug, Clone)]
enum Token {
    ILLEGAL(String),
    EOF(String),
    NEWLINE(String),
    // Delimiters
    LBRACE(String),
    RBRACE(String),
    COLON(String),
    COMMA(String),
    // key:value
    KEY(String),
    VALUE(String),
}

impl Token {
    fn from_char(ch: char) -> Token {
        match ch {
            '{' => Token::LBRACE(ch.to_string()),
            '}' => Token::RBRACE(ch.to_string()),
            '\0' => Token::EOF(String::new()),
            '\n' => Token::NEWLINE(ch.to_string()),
            ':' => Token::COLON(ch.to_string()),
            ',' => Token::COMMA(ch.to_string()),
            _ => Token::ILLEGAL(ch.to_string()),
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a String) -> Lexer {
        let lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
            identifier: "key",
        };
        lexer
    }

    fn read_char(&mut self) -> char {
        let ch = match self.input.chars().nth(self.position) {
            None => '\0',
            Some(x) => x,
        };
        self.position += 1;
        ch
    }

    fn read_identifier(&mut self, first_symbol: char) -> String {
        let identifier = String::new();
        identifier.push(first_symbol);

        let mut current_symbol = self.read_char();
        identifier.push(current_symbol);

        while let current_symbol = '"' {
            current_symbol = self.read_char();
            identifier.push(current_symbol);
        }
        identifier
    }

    fn toggle_current_identifier(&mut self) {
        self.current_identifier = match self.current_identifier {
            "key" => "value",
            "value" => "key",
        };
    }

    fn next_token(&mut self) -> Token {
        let ch = self.read_char();

        if ch == '"' {
            // identify if whether it is a key or value
            let identifier = self.read_identifier(ch);
            let token = match self.current_identifier {
                "key" => Token::KEY(identifier),
                "value" => Token::VALUE(identifier),
            };
            self.toggle_current_identifier();
            return token;
        }
        Token::from_char(ch)
    }

    fn read_key(self) -> Token {}

    fn is_number(ch: char) -> bool {}
}

struct Parser<'a> {
    lexer: &Lexer<'a>,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Parser {
        let parser = Parser {
            lexer: lexer,
            tokens: vec![],
        };
        parser
    }

    fn from_string(content: &'a String) -> Parser {
        let lexer = Lexer::new(&content);
        Parser::new(lexer)
    }

    fn parse(&mut self) {
        loop {
            let token = self.lexer.next_token();
            // Fix it
            self.tokens.push(token.clone());
            match token {
                Token::EOF(_) => break,
                _ => continue,
            };
        }
    }

    fn is_valid(self) -> bool {
        // even if empty string we'll have at least one token
        // moreover we can simplify it to checking count of tokens
        // cause it is not possible to have valid json with one
        // parsed token.
        if matches!(self.tokens.first().unwrap(), Token::EOF(_)) {
            return false;
        }
        for token in self.tokens {
            match token {
                Token::ILLEGAL(_) => return false,
                _ => continue,
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::Path};

    use crate::Parser;

    fn read_file(path: &str) -> String {
        let mut content = String::new();
        File::open(Path::new(path))
            .expect("No file found.")
            .read_to_string(&mut content)
            .expect("error during reading file.");
        content
    }

    #[test]
    fn test_step1_valid() {
        let content = read_file("../test_data/step1/valid.json");
        let mut parser = Parser::from_string(&content);
        parser.parse();
        assert_eq!(true, parser.is_valid());
    }

    #[test]
    fn test_step1_invalid() {
        let content = read_file("../test_data/step1/invalid.json");
        let mut parser = Parser::from_string(&content);
        parser.parse();
        assert_eq!(false, parser.is_valid());
    }

    #[test]
    fn test_step2_valid() {
        let content = read_file("../test_data/step2/valid.json");
        let mut parser = Parser::from_string(&content);
        parser.parse();
        assert_eq!(true, parser.is_valid());
    }

    #[test]
    fn test_step2_invalid() {
        let content = read_file("../test_data/step2/invalid.json");
        let mut parser = Parser::from_string(&content);
        parser.parse();
        assert_eq!(false, parser.is_valid());
    }

    #[test]
    fn test_step2_valid2() {
        let content = read_file("../test_data/step2/valid2.json");
        let mut parser = Parser::from_string(&content);
        parser.parse();
        assert_eq!(true, parser.is_valid());
    }

    #[test]
    fn test_step2_invalid2() {
        let content = read_file("../test_data/step2/invalid2.json");
        let mut parser = Parser::from_string(&content);
        parser.parse();
        assert_eq!(false, parser.is_valid());
    }
}
