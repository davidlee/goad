//! Compiles `ui/app.slint` into the generated module `generated.rs` wraps.
//! Debug info stays on: without it the element query API returns empty and
//! every renderer test passes vacuously (design.md §5.2, §5.5 A-3).

fn main() -> Result<(), Box<dyn std::error::Error>> {
  slint_build::compile_with_config(
    "ui/app.slint",
    slint_build::CompilerConfiguration::new().with_debug_info(true),
  )?;
  Ok(())
}
