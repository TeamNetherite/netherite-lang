use crate::ident::Ident;

#[tokens]
pub enum Pattern {
    Ident(Ident),

}
