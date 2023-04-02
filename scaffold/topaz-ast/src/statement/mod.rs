pub mod variable;
pub mod func_call;

#[tokens]
pub enum Statement {
    Let(variable::LetStmt),
    FuncCall(func_call::FuncCallStmt)
}
