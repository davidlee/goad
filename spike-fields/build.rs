fn main() {
    slint_build::compile_with_config(
        "ui/spike.slint",
        slint_build::CompilerConfiguration::new()
            .with_debug_info(true)
            .with_style("fluent".into()),
    )
    .unwrap();
}
