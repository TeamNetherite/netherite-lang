use string_interner::StringInterner;

pub struct Parser {
    pub(crate) interner: StringInterner
}

impl Parser {
    pub fn new() -> Self {
        Self {
            interner: StringInterner::new()
        }
    }
}