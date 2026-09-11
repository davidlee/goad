//! The one place a line reaches a person's terminal.
//!
//! Not a formatter: what the line *says* belongs to whoever composed it —
//! `crates/goad`'s `diagnostics` for the host, `render` for a command-line
//! binary. This module owns only the last step, and it owns it in stratum 2 so
//! that every entry point spells "best effort" the same way rather than each
//! deciding for itself what an unwritable handle means.

/// Best effort: if the handle cannot be written there is nowhere left to
/// report that, and the exit code still carries the fact. `.ok()` and
/// `drop(..)` both pass the lint table too; this spelling is chosen because
/// it is the only one of the three that says *both outcomes were considered*
/// rather than merely *discarded*.
pub fn line_to(mut sink: impl std::io::Write, line: &str) {
  match writeln!(sink, "{line}") {
    Ok(()) | Err(_) => (),
  }
}

#[cfg(test)]
mod tests {
  use super::line_to;

  /// A sink whose every write fails, so the "best effort" claim is asserted
  /// rather than described.
  struct Broken;

  impl std::io::Write for Broken {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
      Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
    }
    fn flush(&mut self) -> std::io::Result<()> {
      Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
    }
  }

  #[test]
  fn a_line_reaches_the_sink_with_one_terminator() {
    let mut sink = Vec::new();
    line_to(&mut sink, "a line");
    assert_eq!(String::from_utf8(sink).expect("utf-8"), "a line\n");
  }

  #[test]
  fn an_unwritable_sink_is_not_a_panic() {
    line_to(Broken, "nowhere to report that this failed");
  }
}
