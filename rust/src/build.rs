fn main() {
    println!("cargo::rerun-if-changed=macro_utils/src/lib.rs");
    rs::build("macro_utils/src/lib.rs");
}
