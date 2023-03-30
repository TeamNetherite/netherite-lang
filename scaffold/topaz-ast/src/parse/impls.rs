use crate::parse::parse_impl;

parse_impl! {
    crate::visibility::Visibility as VisibilityParser,
    crate::file::TopazFile as FileParser,
}
