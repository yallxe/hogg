fn main() {
    const PROTO_FILE: &str = "proto/daemon.proto";
    
    tonic_build::compile_protos(PROTO_FILE).unwrap();
    println!("cargo:rerun-if-changed={}", PROTO_FILE);
}
