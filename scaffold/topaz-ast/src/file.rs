use crate::item::Item;

#[tokens(no_debug)]
#[derive(Debug, PartialEq, Eq)]
pub struct TopazFile {
    pub items: Vec<Item>
}
