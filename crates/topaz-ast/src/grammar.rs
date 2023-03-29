use crate::ident::Ident;
use crate::path::*;
use oak::oak;
use Option::*;
use crate::item::import::Import;
use crate::token::Token;

fn collect(real: Vec<char>) -> String {
    real.into_iter().collect()
}

fn ident(real: Vec<char>) -> Ident {
    Ident::new(&collect(real))
}
fn path(mut real: Vec<Ident>, last: Ident) -> Path {
    real.push(last);
    Path(real.into_iter().collect())
}

oak! {
    whitespace = [" \n\r\t"]*:(^)

    IMPORT: Token![import] = "import" > Default::default
    LET = "let"
    DOUBLECOLON: (^) = whitespace "::"
    THIS = "this"
    SUPER = "super"

    ident: Ident = !["0-9"] ["a-zA-Z0-9_"]+ > ident

    _mod_path_part = ident DOUBLECOLON
    mod_path: Path = _mod_path_part* ident > path

    import: Import = IMPORT mod_path > Import
}
