use crate::ProgramUnit;
use topaz_macro::_make_visitor;

macro_rules! visitor {
    ($($visit:ident with $fu:tt);*) => {
        $(
        _make_visitor!($fu);
        )*
    }
}
