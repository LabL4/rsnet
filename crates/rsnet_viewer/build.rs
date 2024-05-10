fn main() {
    println!("cargo:rerun-if-changed=assets/shaders");
    println!("cargo:rerun-if-changed=crates/rsnet_net_parser/src/test.py");
}
