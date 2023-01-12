use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Location {
    pub index: usize,
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn start() -> Self {
        Self {
            index: 0,
            line: 1,
            column: 0,
        }
    }

    pub fn advance(&mut self, character_len: usize, newline: bool) {
        if newline {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        self.index += character_len;
    }

    pub fn char_location<'a>(&mut self, filename: &'a Path, character_len: usize) -> Span<'a> {
        Span {
            filename: filename,
            start: *self,
            end: Location {
                index: self.index + character_len,
                line: self.line,
                column: self.column + 1,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span<'a> {
    pub filename: &'a Path,
    pub start: Location,
    pub end: Location,
}

impl<'a> Span<'a> {
    pub fn new(filename: &'a Path, start: Location, end: Location) -> Self {
        Self {
            filename,
            start,
            end,
        }
    }
}
