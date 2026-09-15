//! `crates/goad/src/glass.rs` — stratum 3, and the only file in the crate
//! that names a generated type outside `generated.rs` itself (design.md
//! §5.3).

use std::fmt;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, StyledText, VecModel};

use crate::controller::{Frame, Surface};
use crate::diagnostics::{
  BUSY_NOTICE, Diagnostics, TrayState, next_check_line, report_platform, tooltip, tray_icon,
};
use crate::draft::{Draft, Edited};
use crate::generated::{FieldBlock, FieldRow, OptionRow, PromptWindow, Tray, WindowMode};
use crate::reception::Prepared;
use crate::view_model;
use crate::view_model::Body;

/// Total, and the only method: writing every property, every call, is the
/// design's answer to a display server that fails partway through an
/// update (design.md §5.3, *Ownership*).
pub trait Glass {
  /// Write **every** property the frame carries a value for, then show the
  /// window in the frame's mode or hide it. Total and idempotent. `notice` is
  /// one of those properties: it is written from `frame.notice` on every call,
  /// and this is its only writer anywhere in the renderer (design.md §5.3).
  ///
  /// One declared property is deliberately not written: `Tray::shown`
  /// (`ui/app.slint`). It exists so that `visible` is a binding rather than a
  /// literal the compiler can fold into a constant — E-4's panic trap (F-28) —
  /// and the frame carries no value for it, because the tray is present for
  /// the life of the process. A writer for it would be a new frame field with
  /// nothing to put in it.
  fn present(&mut self, frame: Frame<'_>);
}

/// The production glass. Fields are private; the three handles are strong
/// clones taken in `start` (PHASE-10) and live for the process.
pub struct SlintGlass {
  window: PromptWindow,
  tray: Tray,
  options: Rc<VecModel<OptionRow>>,
}

/// Hand-written because the generated component handles carry no `Debug`,
/// by derive or by impl, and `missing_debug_implementations` is `deny`
/// (`Cargo.toml`, measured).
impl fmt::Debug for SlintGlass {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.debug_struct("SlintGlass").finish_non_exhaustive()
  }
}

impl SlintGlass {
  /// Writes the tray's `image` and `hover-text` **before returning**,
  /// because the tray registers nothing until a non-empty image is
  /// assigned (`builtins.slint:3241-3244`) and the loop's first `present`
  /// happens only after the event loop has started.
  #[must_use]
  pub fn new(window: PromptWindow, tray: Tray, options: Rc<VecModel<OptionRow>>) -> Self {
    tray.set_image(tray_icon(TrayState::Idle));
    tray.set_hover_text(tooltip(&Diagnostics::default(), false).into());
    Self {
      window,
      tray,
      options,
    }
  }
}

impl Glass for SlintGlass {
  /// Infallible: every property setter returns `()`. A `show()` or
  /// `hide()` failure is reported on stderr through
  /// `diagnostics::report_platform` and `present` returns; the process
  /// keeps running, with no de-duplication (design.md §5.3).
  fn present(&mut self, frame: Frame<'_>) {
    self
      .window
      .set_mode(if matches!(frame.surface, Surface::Diagnostics) {
        WindowMode::Diagnostic
      } else {
        WindowMode::Prompt
      });

    let (heading, body, options, degraded) = match frame.shown {
      Some(prepared) => (
        prepared.presentation.title.clone(),
        styled(&prepared.presentation.body),
        option_rows(prepared),
        prepared.presentation.body_is_degraded(),
      ),
      None => (String::new(), StyledText::default(), Vec::new(), false),
    };
    self.window.set_heading(heading.into());
    self.window.set_body(body);
    self.options.set_vec(options);
    self
      .window
      .set_options(ModelRc::from(Rc::clone(&self.options)));
    self.window.set_body_degraded(degraded);
    self.window.set_busy(frame.busy);

    let lines: Vec<SharedString> = frame
      .diagnostics
      .lines()
      .iter()
      .map(SharedString::from)
      .collect();
    self
      .window
      .set_diagnostic_lines(ModelRc::new(VecModel::from(lines)));

    let next_check = frame.next_check.map(next_check_line).unwrap_or_default();
    self.window.set_next_check(next_check.into());

    let notice = if frame.notice { BUSY_NOTICE } else { "" };
    self.window.set_notice(notice.into());

    self.tray.set_image(tray_icon(frame.diagnostics.state()));
    self
      .tray
      .set_hover_text(tooltip(frame.diagnostics, frame.shown.is_some()).into());

    let shown = match frame.surface {
      Surface::Hidden => self.window.hide(),
      Surface::Prompt | Surface::Diagnostics => self.window.show(),
    };
    if let Err(error) = shown {
      report_platform(&error.to_string());
    }
  }
}

/// The body a frame carries, mapped to the type `PromptWindow::set_body`
/// takes. `Body::Rich` is already parsed and is cloned rather than
/// re-parsed — the parse is retained, not repeated (design.md §5.2).
fn styled(body: &Body) -> StyledText {
  match body {
    Body::None => StyledText::default(),
    Body::Plain(text) => StyledText::from_plain_text(text),
    Body::Rich(rich) => rich.clone(),
  }
}

/// One `OptionRow` per retained option, carrying the presentation's
/// `PresentationOption`, the option's drawn fields, and the view token the
/// markup hands back on `chosen` and on `edited` (design.md §5.3, R-14, D10,
/// D19).
///
/// Every row is rebuilt from scratch on every present: nothing here is
/// cached, so nothing has an invalidation rule to get wrong. That is what
/// `Glass::present`'s totality buys, and it is what writes a dropped edit
/// back off the screen.
fn option_rows(prepared: &Prepared) -> Vec<OptionRow> {
  prepared
    .presentation
    .options
    .iter()
    .map(|option| OptionRow {
      id: option.id.as_str().into(),
      label: option.label.as_str().into(),
      view: prepared.view_id.as_str().into(),
      blocks: model(
        option
          .blocks
          .iter()
          .map(|block| field_block(&prepared.draft, option, block))
          .collect(),
      ),
    })
    .collect()
}

/// One generated `FieldBlock` from one `view_model::FieldBlock`, in declared
/// order.
///
/// **The name is two types here, deliberately.** `design.md` §5.2 gives the
/// Slint struct and the mapper's struct the same name, and this file is the
/// one that holds both: generated types keep their bare names, as `OptionRow`
/// does, and the mapper's is path-qualified at its use sites.
///
/// `heading: None` renders `""`, which the markup reads as an **untitled**
/// block rather than a missing one — an ungrouped run is not drawn under a
/// heading that does not claim it.
fn field_block(
  draft: &Draft,
  option: &view_model::PresentationOption,
  block: &view_model::FieldBlock,
) -> FieldBlock {
  FieldBlock {
    heading: block.heading.as_deref().unwrap_or_default().into(),
    fields: model(
      block
        .fields
        .iter()
        .map(|field| {
          // A **lookup**, never stored in the row model as truth: the draft
          // is the authority and this is its projection for one present. The
          // irrefutable `let` is load-bearing — a second `Edited` variant
          // makes it a compile error here, which is where the decision about
          // what a checkbox row shows for a non-boolean value belongs.
          let Edited::Checked(checked) = draft.state_of(&option.id, &field.id);
          FieldRow {
            id: field.id.as_str().into(),
            label: field.label.as_str().into(),
            checked,
          }
        })
        .collect(),
    ),
  }
}

/// A `Vec` as the model the generated array member takes. Named once because
/// the row builders above nest two levels of it.
fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
  ModelRc::new(VecModel::from(items))
}
