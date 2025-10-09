fn main() {
    let proto_files = &["proto/bitcoin_esplora.proto"];
    let includes = &["proto"];

    prost_build::Config::new()
        .compile_protos(proto_files, includes)
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Tell Cargo to recompile if any of the proto files change
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }
}
