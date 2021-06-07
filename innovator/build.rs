fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &["../proto/activity/activity.proto"],
        &["../proto/activity"],
    )?;
    Ok(())
}
