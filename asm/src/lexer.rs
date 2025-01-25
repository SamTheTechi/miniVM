enum State {
    START,
    READCHAR,
    READBLOCK,
    SKIP,
    DUMP,
    COMMENT,
}

pub struct Lexer {
    beg_char: char,
    end_char: char,
    asm_str: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            beg_char: '0',
            end_char: '0',
            asm_str: Vec::new(),
        }
    }

    pub fn lexer(&mut self, string: String) -> Vec<String> {
        let str: Vec<char> = string.chars().collect();
        let mut counter: usize = 0;
        let mut balane_counter: isize = 0;
        let length = str.len();
        let mut lexeme: Vec<char> = Vec::new();
        let mut state = State::START;

        while counter < length {
            match state {
                State::START => {
                    if Self::is_space(&str[counter]) {
                        state = State::SKIP;
                    } else if self.is_grup(&str[counter]) {
                        if str[counter] == self.beg_char {
                            balane_counter = 1;
                            lexeme.push(str[counter]);
                            counter += 1;
                            state = State::READBLOCK;
                        } else {
                            lexeme.push(str[counter]);
                            counter += 1;
                            state = State::READCHAR;
                        }
                    } else if counter + 1 < length && str[counter] == '/' && str[counter + 1] == '/'
                    {
                        counter += 2;
                        state = State::COMMENT;
                    } else {
                        state = State::READCHAR;
                    }
                }
                State::READCHAR => {
                    if Self::is_space(&str[counter]) {
                        state = State::DUMP;
                    } else if self.is_grup(&str[counter]) {
                        if str[counter] == self.beg_char {
                            balane_counter = 1;
                            lexeme.push(str[counter]);
                            counter += 1;
                            state = State::READBLOCK;
                        }
                    } else if counter + 1 < length && str[counter] == '/' && str[counter + 1] == '/'
                    {
                        counter += 2;
                        state = State::COMMENT;
                    } else if counter + 1 < length && str[counter] == ';' {
                        counter += 1;
                        state = State::COMMENT;
                    } else if str[counter] == ':' || str[counter] == ',' {
                        counter += 1;
                    } else {
                        lexeme.push(str[counter]);
                        counter += 1;
                    }
                }
                State::READBLOCK => {
                    if str[counter] == self.beg_char && str[counter] != '"' {
                        balane_counter += 1;
                        lexeme.push(str[counter]);
                        counter += 1;
                    } else if str[counter] == self.end_char {
                        balane_counter -= 1;
                        lexeme.push(str[counter]);
                        counter += 1;
                        if balane_counter <= 0 {
                            state = State::DUMP;
                        }
                    } else if self.beg_char == '"' {
                        if str[counter] == '"' && balane_counter == 0 {
                            lexeme.push(str[counter]);
                            counter += 1;
                            state = State::DUMP;
                        } else if str[counter] == '\\' && counter + 1 < length {
                            lexeme.push(str[counter]);
                            lexeme.push(str[counter + 1]);
                            counter += 2;
                        } else {
                            lexeme.push(str[counter]);
                            counter += 1;
                        }
                    } else {
                        lexeme.push(str[counter]);
                        counter += 1;
                    }
                }
                State::SKIP => {
                    if Self::is_space(&str[counter]) {
                        counter += 1;
                    } else {
                        state = State::READCHAR;
                    }
                }
                State::DUMP => {
                    if !lexeme.is_empty() {
                        self.asm_str.push(lexeme.iter().collect());
                        lexeme.clear();
                    }
                    state = State::START;
                }
                State::COMMENT => {
                    if str[counter] != '\n' {
                        counter += 1;
                    } else {
                        counter += 1;
                        state = State::READCHAR;
                    }
                }
            }
        }
        if !lexeme.is_empty() {
            self.asm_str.push(lexeme.iter().collect());
        }
        self.asm_str.drain(..).collect()
    }

    fn is_space(ch: &char) -> bool {
        match ch {
            '\n' => true,
            '\r' => true,
            '\t' => true,
            ' ' => true,
            _ => false,
        }
    }
    fn is_grup(&mut self, ch: &char) -> bool {
        match ch {
            '"' => {
                self.beg_char = *ch;
                self.end_char = '"';
                true
            }
            '[' => {
                self.beg_char = *ch;
                self.end_char = ']';
                true
            }
            ']' => true,
            _ => false,
        }
    }
}
