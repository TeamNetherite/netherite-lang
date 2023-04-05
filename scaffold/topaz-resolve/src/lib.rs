use topaz_ast::ident::Ident;
use string_interner::{StringInterner, symbol::SymbolU32};
use topaz_ast::path::{AsClause, Path};

pub enum Namespace {
    Core,
    Std,
    Gem(SymbolU32)
}

pub enum ResolvedPath { Path(Namespace, Vec<Ident>, Option<AsClause>) }

#[derive(thiserror::Error, Debug)]
pub enum ResolveError {
    #[error("Namespace not found: {0}")]
    NamespaceNotFound(String),
    #[error("Empty path")]
    EmptyPath,
    #[error("Reference to stdlib path in a no-std context")]
    NoStd
}

pub struct ResolveContext {
    intern: StringInterner,
    has_std: bool,
    gems: Vec<SymbolU32>
}

pub fn resolve(cx: &mut ResolveContext, path: Path) -> Result<ResolvedPath, ResolveError> {

}
