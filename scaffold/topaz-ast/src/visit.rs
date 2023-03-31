use crate::block::Block;
use crate::expr::{Expr, ExprBorrow, ExprConstAccess, ExprLit, ExprVarAccess};
use crate::file::TopazFile;
use crate::ident::Ident;
use crate::item::func::{Func, FuncArg};
use crate::item::import::Import;
use crate::item::type_alias::TypeAlias;
use crate::item::Item;
use crate::literal::number::{BinaryNumber, LiteralNumber};
use crate::literal::{Literal, LiteralString};
use crate::path::{DottedPath, Path};
use crate::pattern::Pattern;
use crate::statement::Statement;
use crate::token::delim::Surround;
use crate::types::{Type, TypeArguments};
use crate::visibility::Visibility;

pub trait Visit: Sized {
    fn visit_file(&mut self, file: &TopazFile) {
        walk_file(self, file);
    }

    fn visit_item(&mut self, item: &Item) {
        walk_item(self, item);
    }

    fn visit_func(&mut self, func: &Func) {
        walk_func(self, func);
    }

    fn visit_type(&mut self, ty: &Type) {
        walk_type(self, ty);
    }

    fn visit_pattern(&mut self, pattern: &Pattern) {
        walk_pattern(self, pattern);
    }

    fn visit_path(&mut self, path: &Path) {
        walk_path(self, path);
    }

    fn visit_ident(&mut self, _ident: &Ident) {
        // noop
    }

    fn visit_type_arguments(&mut self, _type_arguments: &TypeArguments) {
        // noop
    }

    fn visit_func_arg(&mut self, func_arg: &FuncArg) {
        walk_func_arg(self, func_arg);
    }

    fn visit_typealias(&mut self, typealias: &TypeAlias) {
        walk_typealias(self, typealias);
    }

    fn visit_literal(&mut self, literal: &Literal) {
        walk_literal(self, literal);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        walk_expr(self, expr);
    }

    fn visit_expr_literal(&mut self, expr_literal: &ExprLit) {
        walk_expr_literal(self, expr_literal);
    }

    fn visit_expr_borrow(&mut self, expr_borrow: &ExprBorrow) {
        walk_expr_borrow(self, expr_borrow);
    }

    fn visit_expr_const_access(&mut self, expr_const: &ExprConstAccess) {
        walk_expr_const_access(self, expr_const);
    }

    fn visit_expr_var_access(&mut self, expr_var: &ExprVarAccess) {
        walk_expr_var_access(self, expr_var);
    }

    fn visit_dotted_path(&mut self, path: &DottedPath) {
        walk_dotted_path(self, path);
    }

    fn visit_visibility(&mut self, _vis: &Visibility) {
        // noop
    }

    fn visit_number_literal(&mut self, number: &LiteralNumber) {
        walk_number_literal(self, number);
    }

    fn visit_str_literal(&mut self, _str: &LiteralString) {
        // noop
    }

    fn visit_binary_number_literal(&mut self, _bin: &BinaryNumber) {
        // noop
    }

    fn visit_import(&mut self, import: &Import) {
        walk_import(self, import);
    }

    //// Block & Statements
    fn visit_block(&mut self, block: &Block) {
        walk_block(self, block);
    }

    fn visit_statement(&mut self, statement: &Statement) {
        walk_statement(self, statement);
    }
}

pub fn walk_file(visitor: &mut impl Visit, file: &TopazFile) {
    for i in &file.items {
        visitor.visit_item(i);
    }
}

pub fn walk_path(visitor: &mut impl Visit, path: &Path) {
    if let Some(a) = path.0.last() { visitor.visit_ident(a) }
}

pub fn walk_item(visitor: &mut impl Visit, item: &Item) {
    match item {
        Item::Func(func) => visitor.visit_func(func),
        Item::TypeAlias(typealias) => visitor.visit_typealias(typealias),
        Item::Import(import) => visitor.visit_import(import)
    }
}

pub fn walk_func(visitor: &mut impl Visit, Func(_, vis, ident, args, _, ty, block): &Func) {
    visitor.visit_visibility(vis);
    visitor.visit_ident(ident);
    for arg in args {
        visitor.visit_func_arg(arg);
    }
    visitor.visit_type(ty);
    visitor.visit_block(block);
}

pub fn walk_type(visitor: &mut impl Visit, ty: &Type) {
    visitor.visit_type_arguments(ty.type_arguments());
}

pub fn walk_func_arg(visitor: &mut impl Visit, FuncArg(pattern, ty, init): &FuncArg) {
    visitor.visit_pattern(pattern);
    visitor.visit_type(ty);
    if let Some((_, initializer)) = init {
        visitor.visit_expr(initializer);
    }
}

pub fn walk_typealias(visitor: &mut impl Visit, typealias: &TypeAlias) {
    let TypeAlias(vis, _, args, ident, aliased) = typealias;

    visitor.visit_visibility(vis);
    visitor.visit_type_arguments(args);
    visitor.visit_ident(ident);
    visitor.visit_type(&aliased);
}

pub fn walk_pattern(visitor: &mut impl Visit, pattern: &Pattern) {
    match pattern {
        Pattern::Ident(ident) => visitor.visit_ident(ident),
    }
}

pub fn walk_dotted_path(visitor: &mut impl Visit, path: &DottedPath) {
    path.0.iter().for_each(|a| visitor.visit_ident(a))
}

pub fn walk_expr(visitor: &mut impl Visit, expr: &Expr) {
    match expr {
        Expr::FuncCall(call) => (),
        Expr::Literal(literal) => visitor.visit_expr_literal(literal),
        Expr::Borrow(borrow) => visitor.visit_expr_borrow(borrow),
        Expr::ConstAccess(const_access) => visitor.visit_expr_const_access(const_access),
        Expr::VariableAccess(var_access) => visitor.visit_expr_var_access(var_access),
    }
}

pub fn walk_expr_borrow(visitor: &mut impl Visit, ExprBorrow(_, _, borrowed): &ExprBorrow) {
    visitor.visit_expr(&borrowed);
}

pub fn walk_expr_var_access(visitor: &mut impl Visit, ExprVarAccess(ident): &ExprVarAccess) {
    visitor.visit_dotted_path(ident);
}

pub fn walk_expr_const_access(
    visitor: &mut impl Visit,
    ExprConstAccess(const_path): &ExprConstAccess,
) {
    visitor.visit_path(const_path);
}

pub fn walk_expr_literal(visitor: &mut impl Visit, ExprLit(literal): &ExprLit) {
    visitor.visit_literal(literal);
}

pub fn walk_literal(visitor: &mut impl Visit, literal: &Literal) {
    match literal {
        Literal::String(str) => visitor.visit_str_literal(str),
        Literal::Number(number) => visitor.visit_number_literal(number),
        Literal::Char(chr) => todo!("char literals")
    }
}

pub fn walk_number_literal(visitor: &mut impl Visit, number: &LiteralNumber) {
    match number {
        LiteralNumber::Binary(bin) => visitor.visit_binary_number_literal(bin),
    }
}

pub fn walk_import(visitor: &mut impl Visit, Import(path): &Import) {
    visitor.visit_path(path);
}

pub fn walk_block(visitor: &mut impl Visit, Block(Surround(_, statements, _)): &Block) {
    for stmt in statements {
        visitor.visit_statement(stmt);
    }
}

pub fn walk_statement(_visitor: &mut impl Visit, _stmt: &Statement) {
    // TODO
}
