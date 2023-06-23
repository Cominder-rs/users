fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    cfg_if::cfg_if! {
        if #[cfg(feature = "server")] {
            tonic_build::configure()
                .compile(&["./src/v1/auth.proto", "./src/v1/permissions.proto", "./src/v1/users.proto"], &["./src/v1"])?;
        } else if #[cfg(feature = "client")] {
            tonic_build::configure()
                .build_transport(false)
                .build_server(false)
                .compile(&["./src/v1/auth.proto", "./src/v1/permissions.proto", "./src/v1/users.proto"], &["./src/v1"])?;
        } else {
            panic!("Specify \"server\" or \"client\" feature in");
        }
    }

    Ok(())
}
