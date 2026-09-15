//! The reception seam: one consumption point for an `Outcome` (design.md
//! §5.2, plan.md PHASE-05/EX-5).
//!
//! `Outcome` is not `Clone` and carries six owned fields, so something has to
//! choose where it is taken apart. `receive` is that function, and the only
//! one — confirmed by grep, not merely by convention. It calls `present`
//! itself and hands the resulting `undrawn` straight to the diagnostic
//! reducer in the same expression, which is what turns I-2 from a rule an
//! agent must remember into the only path the types admit: a caller cannot
//! reach a `Presentation` here without also handing its `undrawn` to
//! `Diagnostics::of`.

use goad_semantics::protocol::canonical::{Timestamp, ViewId};
use goad_shell::host::{Outcome, Presented};

use crate::diagnostics::{Diagnostics, Reported};
use crate::draft::Draft;
use crate::view_model::{Presentation, Undrawn, present};

/// The interaction the renderer is showing, the token that answers it, and
/// what the person has done to it so far.
///
/// `view_id` is copied from `Presented::view_id`, which is the public half of
/// the pair `Host` mints; `Host`'s own state keeps the authoritative copy.
/// Two copies, one owner.
///
/// The draft sits **inside** this value rather than beside it, which is what
/// makes `absorb`'s three `Shift` arms need no change: a replaced view installs
/// a fresh `Prepared` with an empty draft, a retained one leaves it, a closed
/// one drops it. A draft held beside `shown` would have a state — draft
/// present, view absent — that the type would admit and the fold would have to
/// rule out by hand (`design.md` §5.3).
#[derive(Debug)]
pub struct Prepared {
  pub view_id: ViewId,
  pub presentation: Presentation,
  /// What the person did, keyed by (option, field). Empty at construction:
  /// the protocol carries no `field.value`, so a view arrives with nothing
  /// answered and `Draft::state_of` reports every field as drawn.
  pub draft: Draft,
}

/// What one `Outcome` becomes, once received.
#[derive(Debug)]
pub struct Received {
  /// The view, already mapped, paired with the identity that answers it.
  /// Both halves or neither — the invalid combination is not representable.
  pub prepared: Option<Prepared>,
  /// `true` when `Outcome::failure` was `Some`. The **only** bit of the
  /// diagnostic half the presentation reducer reads, and deliberately not
  /// `cleanup`: cleanup is orthogonal to success and must never select a
  /// presentation transition (design.md §5.4, F-1).
  pub refused: bool,
  /// Resolved on every outcome, failures included.
  pub next_check: Timestamp,
  /// Replaces whatever the surface was holding, wholesale.
  pub diagnostics: Diagnostics,
}

/// The **only** place an `Outcome` is destructured in `crates/goad/src`
/// (plan.md PHASE-05/EX-5). Total, pure and panic-free.
#[must_use]
pub fn receive(outcome: Outcome) -> Received {
  let Outcome {
    view,
    next_check,
    discarded,
    stderr,
    failure,
    cleanup,
  } = outcome;

  let prepared = view.map(
    |Presented {
       view_id,
       view: canonical,
     }| Prepared {
      view_id,
      presentation: present(&canonical),
      draft: Draft::default(),
    },
  );

  let undrawn: &[Undrawn] = match &prepared {
    Some(prepared) => prepared.presentation.undrawn.as_slice(),
    None => &[],
  };
  let refused = failure.is_some();
  let reported = Reported {
    failure,
    cleanup,
    discarded,
    stderr,
  };
  let diagnostics = Diagnostics::of(reported, undrawn);

  Received {
    prepared,
    refused,
    next_check,
    diagnostics,
  }
}
