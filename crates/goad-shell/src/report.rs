//! The one place a line reaches a person's terminal.
//!
//! Not a formatter: what the line *says* belongs to whoever composed it —
//! `crates/goad`'s `diagnostics` for the host, `render` for a command-line
//! binary. This module owns only the last step, and it owns it in stratum 2 so
//! that every entry point spells its answer to an unwritable handle — best
//! effort, or reported to the caller — the same way rather than each deciding
//! for itself what one means.

/// Best effort: if the handle cannot be written there is nowhere left to
/// report that. Right for a line that reports something else — a failure, a
/// refusal — whose subject is not changed by its report being lost; wrong for
/// a line that **is** the answer, where a lost write means the question went
/// unanswered, and such a caller uses [`try_line_to`] instead. `.ok()` and
/// `drop(..)` both pass the lint table too; this spelling is chosen because
/// it is the only one of the three that says *both outcomes were considered*
/// rather than merely *discarded*.
pub fn line_to(sink: impl std::io::Write, line: &str) {
  match try_line_to(sink, line) {
    Ok(()) | Err(_) => (),
  }
}

/// The same line, and whether it arrived — for a caller whose line is the
/// answer, so a failed write is a fact to report rather than to swallow.
/// Flushed before answering: a buffered sink can report its failure only at
/// the flush, and a flush left to the process's exit reports it to nobody.
///
/// # Errors
/// The sink's own, from the write or the flush.
pub fn try_line_to(mut sink: impl std::io::Write, line: &str) -> std::io::Result<()> {
  writeln!(sink, "{line}")?;
  sink.flush()
}

#[cfg(test)]
mod tests {
  use super::{line_to, try_line_to};

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

  /// A sink that accepts every write and refuses the flush — a buffered sink
  /// whose failure surfaces only there, so a line that was never flushed
  /// cannot pass for one that arrived.
  struct RefusesFlush;

  impl std::io::Write for RefusesFlush {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
      Err(std::io::Error::from(std::io::ErrorKind::StorageFull))
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

  #[test]
  fn a_line_that_is_the_answer_reaches_the_sink_with_one_terminator() {
    let mut sink = Vec::new();
    try_line_to(&mut sink, "an answer").expect("a vector accepts every write");
    assert_eq!(String::from_utf8(sink).expect("utf-8"), "an answer\n");
  }

  #[test]
  fn an_unwritable_sink_is_reported_to_a_caller_whose_line_is_the_answer() {
    let error = try_line_to(Broken, "an answer nobody receives").expect_err("the sink refused");
    assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
  }

  /// The flush's own failure is the answer too (010 `review-code.md` F-13):
  /// every write succeeded, and the line still did not arrive.
  #[test]
  fn a_refused_flush_is_reported_to_a_caller_whose_line_is_the_answer() {
    let error =
      try_line_to(RefusesFlush, "an answer still buffered").expect_err("the flush refused");
    assert_eq!(error.kind(), std::io::ErrorKind::StorageFull);
  }
}
