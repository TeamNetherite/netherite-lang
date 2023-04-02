use crate::path::Path;

/// ```tp
/// import gem::real::create_human;
/// ```
#[tokens(_)]
#[derive(Debug, Eq, PartialEq)]
pub struct Import(pub Path);
