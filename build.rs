fn main() {
    #[cfg(target_os = "windows")]
    winres::WindowsResource::new()
        .set_icon("src/assets/sybil.ico")
        .compile()
        .expect("failed to change icon");
}
