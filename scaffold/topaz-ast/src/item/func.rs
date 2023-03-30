use crate::block::Block;
use crate::expr::Expr;
use crate::ident::Ident;
use crate::pattern::Pattern;
use crate::Token;
use crate::types::Type;
use crate::visibility::Visibility;

#[derive(Tokens)]
pub struct Func(pub Token![func], pub Visibility, pub Ident, pub Vec<FuncArg>, pub Token![->], pub Type, pub Block);

#[derive(Tokens)]
pub struct FuncArg(pub Pattern, pub Type, pub Option<(Token![=], Expr)>);
