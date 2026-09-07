//! Fixture for `structure.rs`'s `production_lines` cut: a file whose inline
//! test module is followed by more production code. The scanner must skip the
//! module's body and resume after it — a cut that takes the whole tail would
//! never read `after_the_module` below, and would be indistinguishable from a
//! file that has no such code at all.

fn before_the_module() -> u32 {
  1
}

#[cfg(test)]
mod tests {
  #[test]
  fn the_module_names_its_own_helper() {
    assert_eq!(super::before_the_module() + inside_the_module(), 3);
  }

  /// Unbalanced braces inside literals, which is what desynchronises a brace
  /// count taken over text that still holds string and char contents.
  #[test]
  fn the_module_holds_unbalanced_braces_inside_literals() {
    let opening = "a JSON fragment: {";
    let brace = '{';
    let raw = r#"another one: {"#;
    assert_eq!(opening.len() + raw.len() + brace.len_utf8(), 33);
  }

  fn inside_the_module() -> u32 {
    2
  }
}

fn after_the_module() -> u32 {
  3
}
