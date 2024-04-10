fn main() {
    #[cfg(target_os = "windows")]
    winres::WindowsResource::new()
        .set_icon("src/assets/sybil.ico")
        .set_manifest(
r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
  <asmv3:application>
    <asmv3:windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </asmv3:windowsSettings>
  </asmv3:application>
</assembly>
"#,
        )
        .compile()
        .expect("failed to change icon");
    println!("cargo:rerun-if-env-changed=OODLE");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OODLE").unwrap_or(
            #[cfg(target_os = "windows")]
            "C:/Program Files/Epic Games/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Win64".to_string(),
            #[cfg(target_os = "linux")]
            "/mnt/c/Program Files/Epic Games/UE_5.1/Engine/Source/Runtime/OodleDataCompression/Sdks/2.9.8/lib/Linux".to_string(),
        )
    );
}
