use std::marker::PhantomData;
use topaz_ast::token::Punctuation;

pub enum LexState {
    WaitingItem,
    WaitingStmt,
    WaitingExpr,
    WaitingPunct(PhantomData<&'static dyn Punctuation>)
}
