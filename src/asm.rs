enum State {
    START,
    READCHAR,
    READBLOCK,
    SKIP,
    DUMP,
    COMMENT,
    END,
}

pub struct Assembler {
    asm_str: Box<str>,
    beg_char: char,
    end_char: char,
}

impl Assembler {
    pub fn lexer(&self, str: &str) {
        let length: usize = str.len();
        let mut first_counter: i32 = 0;
        let mut second_counter: i32 = 0;
        let mut state = State::START;
        let a: char = 'a';

        while (first_counter as usize) < length {
            match state {
                State::START => {
                    if Self::is_space(&a) {
                        state = State::SKIP;
                    } else if Self::is_grup(&a) {
                        state = State::READCHAR;
                    } else if a == '/' {
                        first_counter += 2;
                        state = State::READBLOCK;
                    } else {
                        state = State::READCHAR;
                    }
                }
                State::READCHAR => {}
                State::READBLOCK => {}
                State::SKIP => {
                    if Self::is_space(&a) {
                        first_counter += 1;
                    } else {
                        state = State::READCHAR;
                    }
                }
                State::DUMP => {}
                State::COMMENT => {}
                State::END => {
                    first_counter = length as i32;
                }
            }
        }
    }

    fn is_grup(ch: &char) -> bool {
        match ch {
            '[' | ']' => true,
            _ => false,
        }
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
    fn is_special(&mut self, ch: &char) -> bool {
        self.beg_char = *ch;
        match ch {
            '"' => {
                self.end_char = '"';
                true
            }
            '(' => {
                self.end_char = ')';
                true
            }
            ')' => true,
            _ => false,
        }
    }
}
