use crate::expr::Expr;
use crate::ident::Ident;
use crate::pattern::Pattern;
use crate::types::Type;
use crate::visibility::Visibility;

pub struct Func(pub Visibility, pub Ident, pub Vec<FuncArg>, pub Type);

pub struct FuncArg(pub Pattern, pub Type, pub Option<Expr>);
