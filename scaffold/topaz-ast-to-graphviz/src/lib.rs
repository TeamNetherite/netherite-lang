use std::ops::Deref;
use topaz_ast::{location::WithSpan, *};
use topaz_ast::file::TopazFile;
use topaz_ast::item::func::Func;
use topaz_ast::visit::Visit;

pub struct TopazASTToGraphviz {
    current_node_index: u32
}

impl TopazASTToGraphviz {
    pub fn ast_to_graphviz(&mut self, file: &TopazFile) {
        println!("digraph {{");
        self.visit_file(file);
        println!("}}");
    }

    fn add_node(&mut self, label: &str) -> u32 {
        self.current_node_index += 1;

        println!("\tnode{} [label=\"{}\"];", self.current_node_index, label);

        self.current_node_index
    }

    fn add_node_connections(&mut self, connections: &[u32]) {
        print!("\t");
        print!(
            "{}",
            &connections
                .iter()
                .map(|x| format!("node{x}"))
                .collect::<Vec<String>>()
                .join(" -> "),
        );
        println!(";");
    }
}

impl Visit for TopazASTToGraphviz {
    fn visit_func(&mut self, func: &Func) {

    }
}
