pub(crate) struct Scanner {
    start: *const u8,
    current: *const u8,
    line: usize,
}

pub(crate) struct Token {
    token_type: TokenType,
    start: *const u8,
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

impl Scanner {
    pub(crate) fn new(source: &str) -> Self {
        Scanner {
            start: source.as_ptr(),
            current: source.as_ptr(),
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
            b'(' => return self.make_token(TokenType::LeftParen),
            b')' => return self.make_token(TokenType::RightParen),
            b'{' => return self.make_token(TokenType::LeftBrace),
            b'}' => return self.make_token(TokenType::RightBrace),
            b';' => return self.make_token(TokenType::Semicolon),
            b',' => return self.make_token(TokenType::Comma),
            b'.' => return self.make_token(TokenType::Dot),
            b'-' => return self.make_token(TokenType::Minus),
            b'+' => return self.make_token(TokenType::Plus),
            b'/' => return self.make_token(TokenType::Slash),
            b'*' => return self.make_token(TokenType::Star),
            b'!' => {
                if self.check(b'=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            }
            b'=' => {
                if self.check(b'=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            }
            b'<' => {
                if self.check(b'=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            }
            b'>' => {
                if self.check(b'=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            }
            b'"' => { return self.string(); }
            _ => (),
        }

        if self.is_at_end() { return self.make_token(TokenType::Eof); }

        self.error_token("Unexpected character.")
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() { return self.error_token("Unterminated string"); }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek()) { self.advance(); }

        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
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

    fn advance(&mut self) -> u8 {
        self.current = unsafe { self.current.add(1) };
        unsafe { *self.current.offset(-1) }
    }

    fn check(&mut self, expected: u8) -> bool {
        if self.is_at_end() { return false; }
        unsafe { if *self.current != expected { return false; } }
        self.current = unsafe { self.current.add(1) };
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\r' | b'\t' => { self.advance(); }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' => {
                    if self.peek_next() == b'/' {
                        while self.peek() == b'\n' && !self.is_at_end() { self.advance(); }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn peek(&self) -> u8 {
        unsafe { *self.current }
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() { return b'\0' }
        unsafe { *self.current.add(1) }
    }

    fn is_at_end(&self) -> bool {
        unsafe { *self.current == b'\0' }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: unsafe { self.current.offset_from(self.start) as usize },
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token { 
            token_type: TokenType::Error,
            start: message.as_ptr(), 
            length: message.len(), 
            line: self.line,
        }
    }

    fn ident_type(&mut self) -> TokenType {
        match unsafe { *self.start } {
            b'a' => return self.check_keyword(1, 2, "nd", TokenType::And),
            b'c' => return self.check_keyword(1, 4, "lass", TokenType::Class),
            b'e' => return self.check_keyword(1, 3, "lse", TokenType::Else),
            b'i' => return self.check_keyword(1, 1, "f", TokenType::If),
            b'n' => return self.check_keyword(1, 2, "il", TokenType::Nil),
            b'o' => return self.check_keyword(1, 1, "r", TokenType::Or),
            b'p' => return self.check_keyword(1, 4, "rint", TokenType::Print),
            b'r' => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            b's' => return self.check_keyword(1, 4, "uper", TokenType::Super),
            b'v' => return self.check_keyword(1, 2, "ar", TokenType::Var),
            b'w' => return self.check_keyword(1, 4, "hile", TokenType::While),
            b'f' => {
                if self.current as usize - self.start as usize > 1 {
                    match unsafe { *self.start.add(1) } {
                        b'a' => return self.check_keyword(2, 3, "lse", TokenType::False),
                        b'o' => return self.check_keyword(2, 1, "r", TokenType::For),
                        b'u' => return self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => (),
                    }
                }
            }
            b't' => {
                if self.current as usize - self.start as usize > 1 {
                    match unsafe { *self.start.add(1) } {
                        b'h' => return self.check_keyword(2, 2, "is", TokenType::This),
                        b'r' => return self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        TokenType::Ident
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current as usize - self.start as usize == start + length {
            let slice = unsafe { std::slice::from_raw_parts(self.start.add(start), length) };
            if slice == rest.as_bytes() {
                return token_type;
            }
        }
        TokenType::Ident
    }

    fn is_digit(c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    fn is_alpha(c: u8) -> bool {
        c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c == b'_'
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