# A nix devshell font needs `makeFontsConf` + `FONTCONFIG_FILE`, not just `buildInputs`

Learned at slice 002, PHASE-03 (`design.md` D12, `notes.md:1173-1175`).

## The fact

Adding a font package (`pkgs.dejavu_fonts`) to a devshell's `buildInputs`
makes the package present on disk and does nothing else — fontconfig still
cannot find it, because fontconfig resolves fonts through its own config
file and cache, not through the Nix store path being merely available.

The fix is two additional pieces: `pkgs.makeFontsConf` builds a fontconfig
config naming the font package's output, and `FONTCONFIG_FILE` in the
devshell environment points at it.

## Why it matters here

The renderer's tests need a deterministic, test-only font family so
Markdown/text-shaping assertions do not depend on whatever fonts happen to
be installed on a given machine. The real proof this wiring works is not
"the package is in the store" — it is that the test tier does not panic
somewhere in Slint's font-loading stack when it starts up headless.

## How to apply

- Adding any font to a Nix devshell: pair the package with
  `pkgs.makeFontsConf { fontDirectories = [ ... ]; }` and export
  `FONTCONFIG_FILE` pointing at its result, in the same change.
- Verify by running the actual consumer (here, the renderer's test suite)
  rather than checking the package is on `$PATH` or in the store — a font
  package with no fontconfig wiring produces no build error, only a runtime
  failure at first use.
- A second font package in the same devshell is worth a STOP (`design.md`
  Scope) — the wiring above is per-config, not additive for free.
