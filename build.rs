fn main() {
    let mut oodle_path:String = "".to_string();
    use std::path::Path;
    if Path::new("C:/Program Files/Epic Games/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Win64").exists() {
        oodle_path = "C:/Program Files/Epic Games/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Win64".to_string();
    } else {
        oodle_path = "E:/Unreal/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Win64".to_string();
    };
    winres::WindowsResource::new()
        .set_icon("src/assets/sybil.ico")
        .compile()
        .expect("failed to change icon");
    println!("cargo:rerun-if-env-changed=OODLE");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OODLE").unwrap_or(
            oodle_path
        )
    );
}
