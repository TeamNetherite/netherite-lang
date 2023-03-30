fn main() {
    let path = "./src/grammar.lalrpop";
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .always_use_colors()
        .process_file(path)
        .unwrap();
    println!("cargo:rerun-if-changed={}", path)
}
