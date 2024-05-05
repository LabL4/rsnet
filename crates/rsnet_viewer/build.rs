fn main() {
    println!("cargo:rerun-if-changed=assets/shaders");
    println!("cargo:rerun-if-changed=glyphon/");
}
