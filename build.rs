fn main() {
    winres::WindowsResource::new()
        .set_icon("src/assets/sybil.ico")
        .compile()
        .expect("failed to change icon");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OODLE").unwrap_or(
            "C:/Program Files/Epic Games/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Win64".to_string()
        )
    );
}
