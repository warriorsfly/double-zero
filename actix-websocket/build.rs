fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/message/message.proto"], &["proto/message"])?;
    Ok(())
}
