
fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false) // We don't need any of the server code
        .include_file("mod.rs")
        .out_dir("src/proto")
        .compile_protos(
            &["../../src/chat.proto"], 
            &["../../src/"]).unwrap();
}