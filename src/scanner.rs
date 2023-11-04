pub(crate) struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

pub(crate) struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
}

#[derive(PartialEq, Debug)]
pub(crate) enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Ident, String, Num,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, Eof
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        let c = self.advance();
        if Self::is_alpha(c) { return self.ident(); }
        if Self::is_digit(c) { return self.number(); }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                if self.check('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.check('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.check('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            }
            '>' => {
                if self.check('=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            }
            '"' => { return self.string(); }
            _ => (),
        }

        if self.is_at_end() { return self.make_token(TokenType::Eof); }

        self.error_token("Unexpected character.")
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() { return self.error_token("Unterminated string"); }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek()) { self.advance(); }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) { self.advance(); }
        }

        self.make_token(TokenType::Num)
    }

    fn ident(&mut self) -> Token {
        while Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) { self.advance(); }
        let token_type = self.ident_type();
        self.make_token(token_type)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        match self.source.chars().nth(self.current - 1) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn check(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.peek() != expected{ return false; }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() == '\n' && !self.is_at_end() { self.advance(); }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn peek(&self) -> char {
        match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() { return '\0' }

        match self.source.chars().nth(self.current + 1) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn is_at_end(&self) -> bool {
        match self.source.chars().nth(self.current) {
            Some(c) => c == '\0',
            None => true,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token { 
            token_type: TokenType::Error,
            start: self.start, 
            length: message.len(), 
            line: self.line,
        }
    }

    fn ident_type(&mut self) -> TokenType {
        if let Some(c) = self.source.chars().nth(self.start) {
            match c {
                'a' => return self.check_keyword(1, 2, "nd", TokenType::And),
                'c' => return self.check_keyword(1, 4, "lass", TokenType::Class),
                'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
                'i' => return self.check_keyword(1, 1, "f", TokenType::If),
                'n' => return self.check_keyword(1, 2, "il", TokenType::Nil),
                'o' => return self.check_keyword(1, 1, "r", TokenType::Or),
                'p' => return self.check_keyword(1, 4, "rint", TokenType::Print),
                'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
                's' => return self.check_keyword(1, 4, "uper", TokenType::Super),
                'v' => return self.check_keyword(1, 2, "ar", TokenType::Var),
                'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
                'f' => {
                    if self.current - self.start > 1 {
                        if let Some(c) = self.source.chars().nth(self.start +  1) {
                            match c {
                                'a' => return self.check_keyword(2, 3, "lse", TokenType::False),
                                'o' => return self.check_keyword(2, 1, "r", TokenType::For),
                                'u' => return self.check_keyword(2, 1, "n", TokenType::Fun),
                                _ => (),
                            }
                        }
                    }
                }
                't' => {
                    if self.current - self.start > 1 {
                        if let Some(c) = self.source.chars().nth(self.start +  1) {
                            match c {
                                'h' => return self.check_keyword(2, 2, "is", TokenType::This),
                                'r' => return self.check_keyword(2, 2, "ue", TokenType::True),
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        
        TokenType::Ident
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + length {
            let slice = &self.source[self.start+start..self.start+start+length];
            if slice == rest {
                return token_type;
            }
        }
        TokenType::Ident
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }
}

#[test]
fn test_scanner() {
    let source = "!(){} <= == var x = 2.2; class cl for ";

    let exp = [
        TokenType::Bang,
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::LessEqual,
        TokenType::EqualEqual,
        TokenType::Var,
        TokenType::Ident,
        TokenType::Equal,
        TokenType::Num,
        TokenType::Semicolon,
        TokenType::Class,
        TokenType::Ident,
        TokenType::For,
    ];

    let mut scannner = Scanner::new(source);
    for e in exp {
        let t = scannner.scan_token();
        assert_eq!(e, t.token_type);
    }
}