pub mod variable;
pub mod func_call;

#[derive(Tokens)]
pub enum Statement {
    Let(variable::LetStmt),
    FuncCall(func_call::FuncCallStmt)
}
