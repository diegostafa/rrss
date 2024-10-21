use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn to_input_request(evt: &Event) -> Option<InputRequest> {
    use InputRequest::*;
    use KeyCode::*;
    match evt {
        Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            state: _,
        }) if *kind == KeyEventKind::Press => match (*code, *modifiers) {
            (Backspace, KeyModifiers::NONE) | (Char('h'), KeyModifiers::CONTROL) => {
                Some(DeletePrevChar)
            }
            (Delete, KeyModifiers::NONE) => Some(DeleteNextChar),
            (Tab, KeyModifiers::NONE) => None,
            (Left, KeyModifiers::NONE) | (Char('b'), KeyModifiers::CONTROL) => Some(GoToPrevChar),
            (Left, KeyModifiers::CONTROL) | (Char('b'), KeyModifiers::META) => Some(GoToPrevWord),
            (Right, KeyModifiers::NONE) | (Char('f'), KeyModifiers::CONTROL) => Some(GoToNextChar),
            (Right, KeyModifiers::CONTROL) | (Char('f'), KeyModifiers::META) => Some(GoToNextWord),
            (Char('u'), KeyModifiers::CONTROL) => Some(DeleteLine),

            (Char('w'), KeyModifiers::CONTROL)
            | (Char('d'), KeyModifiers::META)
            | (Backspace, KeyModifiers::META)
            | (Backspace, KeyModifiers::ALT) => Some(DeletePrevWord),

            (Delete, KeyModifiers::CONTROL) => Some(DeleteNextWord),
            (Char('k'), KeyModifiers::CONTROL) => Some(DeleteTillEnd),
            (Char('a'), KeyModifiers::CONTROL) | (Home, KeyModifiers::NONE) => Some(GoToStart),
            (Char('e'), KeyModifiers::CONTROL) | (End, KeyModifiers::NONE) => Some(GoToEnd),
            (Char(c), KeyModifiers::NONE) => Some(InsertChar(c)),
            (Char(c), KeyModifiers::SHIFT) => Some(InsertChar(c)),
            (_, _) => None,
        },
        _ => None,
    }
}

pub enum InputRequest {
    SetCursor(usize),
    InsertChar(char),
    GoToPrevChar,
    GoToNextChar,
    GoToPrevWord,
    GoToNextWord,
    GoToStart,
    GoToEnd,
    DeletePrevChar,
    DeleteNextChar,
    DeletePrevWord,
    DeleteNextWord,
    DeleteLine,
    DeleteTillEnd,
}

pub struct StateChanged {
    pub value: bool,
    pub cursor: bool,
}

pub type InputResponse = Option<StateChanged>;

#[derive(Default)]
pub struct Input {
    value: String,
    cursor: usize,
}

impl Input {
    pub fn new(value: String) -> Self {
        let len = value.chars().count();
        Self { value, cursor: len }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.cursor = value.chars().count();
        self.value = value;
        self
    }

    pub fn with_cursor(mut self, cursor: usize) -> Self {
        self.cursor = cursor.min(self.value.chars().count());
        self
    }

    pub fn reset(&mut self) {
        self.cursor = Default::default();
        self.value = Default::default();
    }

    pub fn handle(&mut self, req: InputRequest) -> InputResponse {
        use InputRequest::*;
        match req {
            SetCursor(pos) => {
                let pos = pos.min(self.value.chars().count());
                if self.cursor == pos {
                    None
                } else {
                    self.cursor = pos;
                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }
            InsertChar(c) => {
                if self.cursor == self.value.chars().count() {
                    self.value.push(c);
                } else {
                    self.value = self
                        .value
                        .chars()
                        .take(self.cursor)
                        .chain(std::iter::once(c).chain(self.value.chars().skip(self.cursor)))
                        .collect();
                }
                self.cursor += 1;
                Some(StateChanged {
                    value: true,
                    cursor: true,
                })
            }

            DeletePrevChar => {
                if self.cursor == 0 {
                    None
                } else {
                    self.cursor -= 1;
                    self.value = self
                        .value
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();

                    Some(StateChanged {
                        value: true,
                        cursor: true,
                    })
                }
            }

            DeleteNextChar => {
                if self.cursor == self.value.chars().count() {
                    None
                } else {
                    self.value = self
                        .value
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();
                    Some(StateChanged {
                        value: true,
                        cursor: false,
                    })
                }
            }

            GoToPrevChar => {
                if self.cursor == 0 {
                    None
                } else {
                    self.cursor -= 1;
                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }

            GoToPrevWord => {
                if self.cursor == 0 {
                    None
                } else {
                    self.cursor = self
                        .value
                        .chars()
                        .rev()
                        .skip(self.value.chars().count().max(self.cursor) - self.cursor)
                        .skip_while(|c| !c.is_alphanumeric())
                        .skip_while(|c| c.is_alphanumeric())
                        .count();
                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }

            GoToNextChar => {
                if self.cursor == self.value.chars().count() {
                    None
                } else {
                    self.cursor += 1;
                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }

            GoToNextWord => {
                if self.cursor == self.value.chars().count() {
                    None
                } else {
                    self.cursor = self
                        .value
                        .chars()
                        .enumerate()
                        .skip(self.cursor)
                        .skip_while(|(_, c)| c.is_alphanumeric())
                        .find(|(_, c)| c.is_alphanumeric())
                        .map(|(i, _)| i)
                        .unwrap_or_else(|| self.value.chars().count());

                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }

            DeleteLine => {
                if self.value.is_empty() {
                    None
                } else {
                    let cursor = self.cursor;
                    self.value = "".into();
                    self.cursor = 0;
                    Some(StateChanged {
                        value: true,
                        cursor: self.cursor == cursor,
                    })
                }
            }

            DeletePrevWord => {
                if self.cursor == 0 {
                    None
                } else {
                    let remaining = self.value.chars().skip(self.cursor);
                    let rev = self
                        .value
                        .chars()
                        .rev()
                        .skip(self.value.chars().count().max(self.cursor) - self.cursor)
                        .skip_while(|c| !c.is_alphanumeric())
                        .skip_while(|c| c.is_alphanumeric())
                        .collect::<Vec<char>>();
                    let rev_len = rev.len();
                    self.value = rev.into_iter().rev().chain(remaining).collect();
                    self.cursor = rev_len;
                    Some(StateChanged {
                        value: true,
                        cursor: true,
                    })
                }
            }

            DeleteNextWord => {
                if self.cursor == self.value.chars().count() {
                    None
                } else {
                    self.value = self
                        .value
                        .chars()
                        .take(self.cursor)
                        .chain(
                            self.value
                                .chars()
                                .skip(self.cursor)
                                .skip_while(|c| c.is_alphanumeric())
                                .skip_while(|c| !c.is_alphanumeric()),
                        )
                        .collect();

                    Some(StateChanged {
                        value: true,
                        cursor: false,
                    })
                }
            }

            GoToStart => {
                if self.cursor == 0 {
                    None
                } else {
                    self.cursor = 0;
                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }

            GoToEnd => {
                let count = self.value.chars().count();
                if self.cursor == count {
                    None
                } else {
                    self.cursor = count;
                    Some(StateChanged {
                        value: false,
                        cursor: true,
                    })
                }
            }

            DeleteTillEnd => {
                self.value = self.value.chars().take(self.cursor).collect();
                Some(StateChanged {
                    value: true,
                    cursor: false,
                })
            }
        }
    }

    /// Get a reference to the current value.
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /// Get the currect cursor placement.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Get the current cursor position with account for multispace characters.
    pub fn visual_cursor(&self) -> usize {
        if self.cursor == 0 {
            return 0;
        }

        // Safe, because the end index will always be within bounds
        unicode_width::UnicodeWidthStr::width(unsafe {
            self.value.get_unchecked(
                0..self
                    .value
                    .char_indices()
                    .nth(self.cursor)
                    .map_or_else(|| self.value.len(), |(index, _)| index),
            )
        })
    }

    /// Get the scroll position with account for multispace characters.
    pub fn visual_scroll(&self, width: usize) -> usize {
        let scroll = (self.visual_cursor()).max(width) - width;
        let mut uscroll = 0;
        let mut chars = self.value().chars();

        while uscroll < scroll {
            match chars.next() {
                Some(c) => {
                    uscroll += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                }
                None => break,
            }
        }
        uscroll
    }
}

impl From<Input> for String {
    fn from(input: Input) -> Self {
        input.value
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
