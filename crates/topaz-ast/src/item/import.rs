use crate::path::Path;

make_token! {
    pub struct Import(pub import, pub Path);
}
