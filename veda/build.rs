fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["../proto/message/grpc.proto"], &["../proto/message"])?;
    Ok(())
}
