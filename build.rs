fn main() {
    winres::WindowsResource::new()
        .set_icon("src/umby.ico")
        .compile()
        .expect("failed to change icon")
}
