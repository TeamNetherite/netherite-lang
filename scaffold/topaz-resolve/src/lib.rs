use topaz_ast::ident::Ident;
use string_interner::{StringInterner, symbol::SymbolU32};
use topaz_ast::path::Path;

pub enum Namespace {
    Core,
    Std,
    Gem(SymbolU32)
}

pub struct ResolvedPath(Namespace, Vec<Ident>);

#[derive(thiserror::Error, Debug)]
pub enum ResolveError {
    #[error("Namespace not found: {0}")]
    NamespaceNotFound(Ident),
    #[error("Empty path")]
    EmptyPath,
    #[error("Reference to stdlib path in a no-std context")]
    NoStd
}

pub struct ResolveContext {
    intern: StringInterner,
    has_std: bool
}

pub fn resolve(cx: &mut ResolveContext, path: Path) -> Result<ResolvedPath, ResolveError> {
    let namespace = match path.0.iter().next().ok_or_else(|| ResolveError::EmptyPath)?.value() {
        "std" => if cx.has_std { Namespace::Std } else { return Err(ResolveError::NoStd) },
        "core" => Namespace::Core,
        other => Namespace::Gem(cx.intern.get_or_intern(other))
    };

    
}
