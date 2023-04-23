#[cfg(feature = "server")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["./src/v1/users.proto"], &["./src/v1"])?;
    Ok(())
}

#[cfg(feature = "client")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(false)
        .build_server(false)
        .compile(&["./src/v1/users.proto"], &["./src/v1"])?;
    Ok(())
}