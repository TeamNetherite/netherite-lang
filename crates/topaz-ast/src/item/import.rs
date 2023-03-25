use crate::path::Path;

#[derive(Tokens)]
pub struct Import(pub Token![import], pub Path);
