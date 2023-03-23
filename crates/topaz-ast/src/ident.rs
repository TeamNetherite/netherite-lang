use std::rc::Rc;
use std::sync::Arc;
use string_interner::symbol::SymbolU32;
use crate::_Tokens;
use crate::parser::Parser;

pub struct Ident(SymbolU32, Arc<Parser>);

impl Ident {
    pub fn new(parser: &Arc<Parser>, value: &str) -> Self {
        let parser = Arc::clone(parser);
        Ident(parser.interner.get_or_intern(value), parser)
    }

    pub fn value(&self) -> String {
        Arc::clone(&self.1).interner.resolve(self.0).expect("symbol was dropped").to_owned()
    }
}

impl _Tokens for Ident {}
