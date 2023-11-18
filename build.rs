fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/staff_service.proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("./src/generated")
        .compile(&[proto_file],&["."])
        .unwrap_or_else(|e|panic!("protobuf compile error{}",e));
    Ok(())
}