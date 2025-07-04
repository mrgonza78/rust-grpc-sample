
fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .include_file("mod.rs")
        .out_dir("src/proto")
        .compile_protos(
            &["src/chat.proto"], 
            &["src/"]).unwrap();
}
