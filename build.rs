use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .out_dir("src/")
        .compile_protos(&["proto/zkp_auth.proto"], &["proto/"])?;
    Ok(())
}
