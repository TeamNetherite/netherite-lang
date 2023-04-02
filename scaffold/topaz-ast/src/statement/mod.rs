pub mod variable;
pub mod func_call;

#[tokens]
#[derive(Eq, PartialEq)]
pub enum Statement {
    Let(variable::LetStmt),
    FuncCall(func_call::FuncCallStmt)
}
