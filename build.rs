fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./staffserver/v1/server.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./staffserver/v1/")
        .compile(&[proto_file],&["."])
        .unwrap_or_else(|e|panic!("protobuf compile error{}",e));
    Ok(())
}