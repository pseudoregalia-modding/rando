fn main() {
    winres::WindowsResource::new()
        .set_icon("src/assets/sybil.ico")
        .compile()
        .expect("failed to change icon")
}
