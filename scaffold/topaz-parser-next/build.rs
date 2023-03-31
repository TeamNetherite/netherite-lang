fn main() {
    let path = "./src/grammar.lalrpop";
    lalrpop::Configuration::new()
        .always_use_colors()
        .set_out_dir(std::env::var("OUT_DIR").unwrap())
        .process_file(path)
        .unwrap();
    println!("cargo:rerun-if-changed={}", path)
}
