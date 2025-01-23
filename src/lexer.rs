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
        let mut first_counter: usize = 0;
        let mut second_counter: usize = 0;
        let mut balane_counter: usize = 0;
        let length = str.len();
        let mut lexeme: Vec<char> = Vec::new();
        let mut state = State::START;

        while first_counter < length {
            match state {
                State::START => {
                    if Self::is_space(&str[first_counter]) {
                        state = State::SKIP;
                    } else if self.is_grup(&str[first_counter]) {
                        if str[first_counter] == '"' {
                            lexeme[second_counter] = str[first_counter];
                            first_counter += 1;
                            second_counter += 1;
                        }
                        state = State::READBLOCK;
                    } else if first_counter + 1 < length
                        && str[first_counter] == '/'
                        && str[first_counter + 1] == '/'
                    {
                        first_counter += 2;
                        state = State::COMMENT;
                    } else {
                        state = State::READCHAR;
                    }
                }
                State::READCHAR => {
                    if Self::is_space(&str[first_counter]) {
                        state = State::DUMP;
                    } else if self.is_grup(&str[first_counter]) {
                        if str[first_counter] == '"' {
                            lexeme[second_counter] = str[first_counter];
                            first_counter += 1;
                            second_counter += 1;
                        }
                    } else if first_counter + 1 < length
                        && str[first_counter] == '/'
                        && str[first_counter + 1] == '/'
                    {
                        first_counter += 2;
                        state = State::COMMENT;
                    } else {
                        lexeme.push(str[first_counter]);
                        first_counter += 1;
                        second_counter += 1;
                    }
                }
                State::READBLOCK => {
                    if str[first_counter] == self.beg_char && str[first_counter] != '"' {
                        balane_counter += 1;
                        lexeme.push(str[first_counter]);
                        first_counter += 1;
                        second_counter += 1;
                    } else if str[first_counter] == self.end_char {
                        balane_counter -= 1;
                        lexeme.push(str[first_counter]);
                        first_counter += 1;
                        second_counter += 1;
                        if balane_counter <= 0 {
                            state = State::DUMP;
                        }
                    } else if self.end_char == '"' && str[first_counter] == '\\' {
                        first_counter += 2;
                    } else {
                        lexeme.push(str[first_counter]);
                        first_counter += 1;
                        second_counter += 1;
                    }
                }
                State::SKIP => {
                    if Self::is_space(&str[first_counter]) {
                        first_counter += 1;
                    } else {
                        state = State::READCHAR;
                    }
                }
                State::DUMP => {
                    if second_counter > 0 {
                        self.asm_str.push(lexeme.iter().collect());
                        lexeme.clear();
                        second_counter = 0;
                    }
                    state = State::START;
                }
                State::COMMENT => {
                    if str[first_counter] != '\n' {
                        first_counter += 1;
                    } else {
                        first_counter += 1;
                        state = State::READCHAR;
                    }
                }
            }
        }
        if second_counter > 0 {
            lexeme[second_counter] = '\0';
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
            '(' => {
                self.beg_char = *ch;
                self.end_char = ')';
                true
            }
            ')' => true,
            _ => false,
        }
    }
}
