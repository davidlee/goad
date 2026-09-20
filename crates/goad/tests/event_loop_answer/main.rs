//! **`serve` must engage with the exchange it actually started** —
//! `review-code.md` **F-B1**.
//!
//! `Controller::engage` maps `Exchanged::Answer` to `engaged` and everything
//! else to not-engaged, and slice 003's double-submit guard, AC-4 and AC-5 all
//! read the result. **The mapping is held nine times over and the argument
//! that produces it was held nowhere**: every case that arranges `busy` calls
//! `Controller::engage` itself, so replacing `serve`'s own
//! `controller.engage(exchanged)` with `engage(Exchanged::Evaluation)` — the
//! production loop never engaging at all — left the entire `-p goad` suite
//! green.
//!
//! `event_loop_drain` catches the *opposite* mutation, because its arrangement
//! is a key typed during an evaluation and hardcoding `Answer` disables the
//! form under it. This is the counterpart it names: **the person's own answer
//! in flight, under the production `serve`**, which is the only arrangement in
//! which the narrowed `busy` is supposed to be true.
//!
//! A `[[test]]` target of its own for the reason every `event_loop_*` target
//! here is one: `i_slint_backend_testing`'s per-process init "can only be
//! called once per process" (its own doc comment, D-12). The harness is
//! `event_loop_drain`'s, held to the same shape deliberately — a held backend
//! rather than a scripted one, because the case needs an exchange that is
//! *provably* still outstanding while it reads.
#[cfg(test)]
mod answer;
