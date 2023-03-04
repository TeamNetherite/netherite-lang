//! `precedence.rs` - defines `Precedence` enum for different infix expression operator precedences.
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive)]
pub enum Precedence {
    Lowest,
    // a = b | a += b | a -= b | a *= b | a /= b | a ^= b | a |= b
    Assign,
    // a || b
    OrOr,
    // a && b
    AndAnd,
    // a | b
    Or,
    // a ^ b
    Xor,
    // a & b
    And,
    // a == b | a != b
    Eq,
    // a > b | a < b | a >= b | a <= b
    LessOrGreater,
    // a >> b | a << b
    LeftRightShift,
    // a + b | a - b
    Sum,
    // a * b | a / b
    Product,
    // a ** b
    Power,
    // a % b
    Mod,
    // !a | a?
    PrefixOrPostfix,
    // ?:
    Elvis,
    // a()
    Call,
    // a[0], a.b
    Index,
    // a as i32
    As,
    // a$<i32>()
    Dollar,
}
