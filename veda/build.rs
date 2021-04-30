fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["../proto/message/message.proto"], &["../proto/message"])?;
    Ok(())
}
