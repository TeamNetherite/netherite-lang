use crate::block::Block;
use crate::expr::Expr;
use crate::ident::Ident;
use crate::pattern::Pattern;
use crate::Token;
use crate::types::Type;
use crate::visibility::Visibility;

#[tokens]
#[derive(Eq, PartialEq)]
pub struct Func(pub Token![func], pub Visibility, pub Ident, pub Vec<FuncArg>, pub Option<(Token![->], Type)>, pub Block);

#[tokens]
#[derive(Eq, PartialEq)]
pub struct FuncArg(pub Pattern, pub Type, pub Option<(Token![=], Expr)>);
