fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/pb")
        .build_server(false)
        .compile(&["proto/sign.proto"], &["./"])?;

    tauri_build::build();
    Ok(())
}
