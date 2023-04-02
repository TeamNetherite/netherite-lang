use crate::ident::Ident;

#[tokens]
#[derive(Eq, PartialEq)]
pub enum Pattern {
    Ident(Ident),

}
