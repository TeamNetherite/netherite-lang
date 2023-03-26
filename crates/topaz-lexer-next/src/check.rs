pub const NOT_IN_STRING_CHARS: [char; 2] = [
    '\\',
    '"'
];

pub fn is_in_string_char(c: char) -> bool {
    !NOT_IN_STRING_CHARS.contains(&c)
}
