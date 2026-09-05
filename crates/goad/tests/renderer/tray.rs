//! design.md §9 item 16 (AC-14's neighbour): the tray icon is a rule, and
//! the rule has numbers. `tray_icon` is pure and needs no event loop, no
//! component and no display — its whole test surface is this module.

use std::path::{Path, PathBuf};

use goad::diagnostics::{TrayState, tray_icon};

/// The alpha channel at physical pixel `(x, y)`, read back from the
/// straight-alpha RGBA8 buffer `tray_icon` returns.
fn alpha_at(state: TrayState, x: u32, y: u32) -> u8 {
  let image = tray_icon(state);
  let buffer = image
    .to_rgba8()
    .expect("tray_icon returns a straight-alpha RGBA8 buffer");
  assert_eq!(buffer.width(), 32, "ICON_EDGE is 32");
  assert_eq!(buffer.height(), 32, "ICON_EDGE is 32");
  let index = usize::try_from(y * buffer.width() + x).expect("a 32x32 index fits usize");
  buffer
    .as_slice()
    .get(index)
    .expect("(x, y) is within the 32x32 buffer")
    .a
}

/// VT-3 / item 16: the centre pixel — form, not hue, so the pair survives a
/// monochrome panel and a colour-blind viewer. `Idle` is an annulus (its
/// own centre uncovered); `Fault` is a filled disc.
#[test]
fn the_centre_pixel_is_transparent_for_idle_and_opaque_for_fault() {
  assert_eq!(alpha_at(TrayState::Idle, 16, 16), 0);
  assert_eq!(alpha_at(TrayState::Fault, 16, 16), 255);
}

/// A pixel on the idle ring — 12px from centre, straddling neither the
/// inner nor outer boundary — is fully opaque in both states.
#[test]
fn a_ring_pixel_is_opaque_in_both_states() {
  assert_eq!(alpha_at(TrayState::Idle, 16, 4), 255);
  assert_eq!(alpha_at(TrayState::Fault, 16, 4), 255);
}

/// A corner pixel, outside the outer radius entirely, is transparent in
/// both states.
#[test]
fn a_corner_pixel_is_transparent_in_both_states() {
  assert_eq!(alpha_at(TrayState::Idle, 0, 0), 0);
  assert_eq!(alpha_at(TrayState::Fault, 0, 0), 0);
}

/// AC-14's neighbour: an asset that exists is an asset that can rot. There
/// is no icon file anywhere in this crate — the rule has numbers, not a
/// checked-in image.
#[test]
fn no_file_under_the_crate_is_an_image() {
  let extensions = ["png", "svg", "ico", "bmp", "jpg", "jpeg"];
  let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
  let mut images = Vec::new();
  find_images(crate_root, &extensions, &mut images);
  assert!(
    images.is_empty(),
    "found image files under the crate: {images:?}"
  );
}

fn find_images(dir: &Path, extensions: &[&str], found: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    let skip_generated = path.file_name().is_some_and(|name| name == "target");
    if path.is_dir() {
      if !skip_generated {
        find_images(&path, extensions, found);
      }
      continue;
    }
    let is_image = path
      .extension()
      .and_then(|extension| extension.to_str())
      .is_some_and(|extension| {
        extensions
          .iter()
          .any(|candidate| extension.eq_ignore_ascii_case(candidate))
      });
    if is_image {
      found.push(path);
    }
  }
}
