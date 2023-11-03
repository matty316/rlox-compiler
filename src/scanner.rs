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

pub(crate) enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS,
    SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    // Keywords.
    AND, CLASS, ELSE, FALSE,
    FOR, FUN, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS,
    TRUE, VAR, WHILE,

    ERROR, EOF
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
        self.start = self.current;

        if self.is_at_end() { return self.make_token(TokenType::EOF); }

        return self.error_token("Unexpected character.");
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
            token_type: TokenType::ERROR,
            start: message.as_ptr(), 
            length: message.len(), 
            line: self.line,
        }
    }
}