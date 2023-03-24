use crate::file::TopazFile;
use crate::item::func::Func;
use crate::item::type_alias::TypeAlias;
use crate::item::Item;

macro visitor($($name:ident {$($real:ident: $real_type:ty),*});*) {
    $(
    pub trait $name : Sized {
        $(
        fn visit_$real(&mut self, $real: $real_type) {
            walk_$real(self, $real);
        }
        )*
    }
    )*
}

visitor! {
    Visit {
        file: &TopazFile,
        item: &Item
    };
    VisitMut {}
}

pub fn walk_file<V: Visit>(visitor: &mut V, file: &TopazFile) {
    for i in &file.items {
        visitor.visit_item(i);
    }
}

pub fn walk_item<V: Visit>(visitor: &mut V, item: &Item) {
    match item {
        Item::Func(func) => visitor.visit_func(func),
        Item::TypeAlias(typealias) => visitor.visit_item_typealias,
    }
}

pub fn walk_func<V: Visit>(visitor: &mut V, item_func: &Func) {}

pub fn walk_typealias<V: Visit>(visitor: &mut V, typealias: &TypeAlias) {}
