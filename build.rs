fn main() {
    #[cfg(target_os = "windows")]
    winres::WindowsResource::new()
        .set_icon("src/assets/sybil.ico")
        .compile()
        .expect("failed to change icon");
    println!("cargo:rerun-if-env-changed=OODLE");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OODLE").unwrap_or(
            #[cfg(target_os = "windows")]
            "E:/Unreal/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Win64".to_string(),
            #[cfg(target_os = "linux")]
            "/mnt/c/Program Files/Epic Games/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Linux".to_string(),
        )
    );
}
