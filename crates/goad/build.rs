//! Compiles `ui/app.slint` into the generated module `generated.rs` wraps.
//! Debug info stays on: without it the element query API returns empty and
//! every renderer test passes vacuously (design.md §5.2, §5.5 A-3).
//!
//! The style is a **default**, not a fixture: `SLINT_STYLE=fluent` still
//! reverts it. The read is explicit because `CompilerConfiguration::new()`
//! already takes that variable and `with_style` overwrites what it took
//! (`slint-build-1.17.1/lib.rs:153-157`, `i-slint-compiler-1.17.1/lib.rs:264`),
//! so the one-line form would silently remove an override that works today.
//!
//! **Set-but-empty is unset.** `SLINT_STYLE=` is `Ok("")`, and `with_style("")`
//! does not fall back — measured, it fails the build outright: *"Style  is not
//! known. Use one of the builtin styles […]"*, and then every `std-widgets`
//! element in `app.slint` is unknown because the import cannot be resolved. An
//! empty override is an override that was never made, so it takes the default
//! the same way an absent one does.

fn main() -> Result<(), Box<dyn std::error::Error>> {
  #[expect(
    clippy::disallowed_methods,
    reason = "the ban is on run-time configuration reached through the \
              environment — \"use typed configuration loading instead\" \
              (`clippy.toml`). This runs at build time, in cargo's own \
              environment, and reads the one variable `slint-build` already \
              reads and already declares `cargo:rerun-if-env-changed` for \
              (`slint-build-1.17.1/lib.rs:532`). Reading it here is what keeps \
              that override working, because `with_style` overwrites what \
              `CompilerConfiguration::new()` took; the alternative is not a \
              typed loader but silently dropping the override. `var_os` would \
              pass the lint and evade it, which is worse than recording it."
  )]
  let style = std::env::var("SLINT_STYLE")
    .ok()
    .filter(|style| !style.is_empty())
    .unwrap_or_else(|| "material".into());
  slint_build::compile_with_config(
    "ui/app.slint",
    slint_build::CompilerConfiguration::new()
      .with_debug_info(true)
      .with_style(style),
  )?;
  Ok(())
}
