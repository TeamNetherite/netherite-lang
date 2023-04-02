use crate::item::Item;

#[tokens(no_debug)]
#[derive(Debug)]
pub struct TopazFile {
    pub items: Vec<Item>
}
