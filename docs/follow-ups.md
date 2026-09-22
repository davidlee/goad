# Follow-ups

Every follow-up raised by a closed slice and still open. **This file is the live
list.** A slice's `slice-nnn.md` §Follow-ups records what that slice *raised*,
with the reasoning and the price — it does not go stale, because it is a claim
about what was decided then. This file records what is still *open*, which does.

Not canon. `docs/specs/`, `docs/policy/` and `docs/adr/` govern.

## How a row works

Four parts, and the fourth is the one that earns the file:

- an **id**, immutable and append-only. Reuse none; a dead row is struck, not
  deleted, so a reader who remembers it finds what killed it.
- the **slices that raised it**. Merged — one row per claim, however many slices
  arrived at it. A row citing three slices is three slices agreeing, not three
  follow-ups.
- the **claim**, cited by symbol.
- **Dead when** — the grep, the symbol, or the event that would prove it closed.

Rows carry **threat** and **cost** because those are facts about the row. They
do **not** carry their rank: the ordering is a judgement about the whole set,
re-derived at each sweep, and storing it per row is how a ranking goes stale the
way the entries it ranks do. The bands below are that ordering as it stands.

## Sweeping

`docs/AGENTS.md` §Close requires a closing slice to re-verify the rows naming a
file it touched — bounded, and mechanical because each row carries its own kill
condition. A full sweep is a research task and should be rare.

**Last full sweep: 2026-09-23, at `3ecaa11`.** Sixty-nine claims across nine
slices; eleven dead on verification, the rest merged to the rows below. The
eleven are struck under §Closed, with what killed each one.

---

## Band 1 — measured harm on the running host

Evidence of the host failing on the machine it runs on.

### FU-1 — the exit-code taxonomy
**Raised by** 006 · **cost** tier 2, `SPEC-003`'s failure vocabulary · **scheduled as slice 010**

`start` ends `run_event_loop_until_quit().map_err(StartupError::Platform)`, so a
compositor going away under a host that has run for hours exits 2 exactly as a
host that never started does. `nix/module.nix`'s `RestartPreventExitStatus`
argues itself from variants that cannot succeed on a retry; `Platform` is the
counterexample and the only exit-2 that has actually happened. Four such exits
in two days, two of them leaving the host down for around two hours. The repair
reaches the startup surface, `main`'s single exit decision, and SPEC-003's
failure vocabulary.

**Dead when** `main`'s exit decision separates *never started* from *stopped
running*, and `nix/module.nix` no longer names `Platform` as a known exception
to its own directive.

---

## Band 2 — reachable without anyone misbehaving

A conforming backend, or a legitimate writer, walks into it. This is where the
invariant *a backend failure never takes the host down* is thinnest.

### FU-2 — what a refused ingress arrival costs
**Raised by** 009 (`F-R4`) · **cost** tier 2, a `SPEC-003/R-15` amendment with its own verification · **scheduled as slice 011**

One full present — `show()`'s instantiation pass included — per **refused**
arrival, at a rate an untrusted writer sets. Suppressing it defers the refusal
to the next scheduled firing, which is what R-15 requires reach the diagnostics
surface; and R-15's own verification case reads the retained model rather than
the window, so canon's instrument would not report the change. Splitting
`option_models`' single walk would reintroduce the second counter invariant
**I-B** forbids — so that is not the route.

**Dead when** a refused arrival no longer costs a present, and R-15 plus its
verification row say what a refusal decided while idle is now owed.

### FU-3 — what the diagnostics slot holds
**Raised by** 001, 004 (three entries) · **cost** tier 2 — it reaches `SPEC-003` OQ-3 and three refusal paths that predate ingress

`Diagnostics::refused` replaces the whole retained value, and ingress is an
unbounded author of it from outside the process — measured at roughly 1690/s,
which puts a backend failure a person needs to see out of reach within a
millisecond. `Outcome` is per-call and forgotten, so nothing decides retention.
Four refusals never reach a person at all, `engaged` among them — the commonest
a real watcher will meet, and invisible to whoever is debugging that watcher.
An accepted ingested evaluation is not distinguishable on the surface either
(`SPEC-003` OQ-3). One decision — retention, and precedence between
host-authored faults and externally-triggered refusals — reaching
`SupersededView`, `UnknownOption` and `NoClock`, so it cannot be settled inside
the ingress arm.

**Severity decays.** 004 raised this `minor` on the ground that the single-slot
surface predates that slice. That ground does not survive a second deferral.

**Dead when** the surface states what it retains and in what precedence, and
`engaged` is reachable by the person debugging the writer that provoked it.

### FU-4 — what bound a backend-authored string gets
**Raised by** 007 (F-4, F-16), 008 (L-7) · **cost** tier 1 — a decision about the look, no canon

`title`, `option.label` and `block.heading` bind straight to a `Text` with no
bound, while every diagnostic line in the same binary passes
`finish(.., LINE_LIMIT)`. Not an injection risk — Slint's `Text` interprets no
markup — the exposure is layout, and a backend need not misbehave to reach it.
The same decision covers `Diagnostics::of`'s unbounded **lists**: the undrawn
report is linear in an option's undrawn fields, `discarded` is unbounded the
same way, and R-15 bounds neither. What an *"and N more"* line means when the
list rather than the line overflowed is part of it.

**Dead when** a backend-authored string reaching the screen gets the treatment
one reaching the diagnostics pane gets, taken beside the bounds
`diagnostics.rs` already states rather than added in passing.

### FU-5 — stratum-3 test helpers have no shared home
**Raised by** 006, 007 (F-17, F-22) · **cost** tier 1

`claim` reaches four of the six helpers that promise a unique temp path. Both
stragglers are `goad-emit`'s: `tests/binary/exchange.rs`'s `socket_path`, and
`src/main.rs`'s `config_home`, which is inside a `#[cfg(test)]` module in a
production file and is **destructive** — it writes or removes `config.toml`
under the directory as it hands it out, so two cases sharing a name have one
clobber the other's fixture mid-run. Measured at one failure in six under a
deliberate collision, with no panic and no name in the message. The obstacle is
measured: neither target `#[path]`-includes `tests/support/scripting.rs`, and
adding the include yields `dead_code` warnings that are errors under the gate.
Same cause as the four binary-tier helpers transcribed into both
`goad-emit/tests/binary/exchange.rs` and `goad/tests/binary/process.rs` —
nothing at stratum 3 is shared and neither crate may depend on the other.

**Start from the grep, not from a list**: `claim`'s own doc carries the class
and the grep that finds it (`process::id()`). The enumeration came up short
twice.

**Dead when** every helper promising a unique path goes through `claim`, from a
support file a target can include without the scripted-backend helpers.

### FU-6 — nothing re-probes the socket path once bound
**Raised by** 004 · **cost** tier 1

A socket unlinked or replaced underneath a live listener leaves the host holding
a descriptor no `connect` can reach, and it has nothing to report because
nothing arrives (`SPEC-003` §6.1, *the path after the bind*). The advisory lock
does not reach this: it keeps a **second** host off the path, and this is about
the **first** host's own socket. What else is left of the original
single-instance question: nothing bounds two goad processes against *different*
paths, or against none, and there is no pidfile.

**Dead when** a live host notices its own path has gone out from under it.

---

## Band 3 — nothing holds it

No harm today. This is how the next defect gets in.

**Three of these share one price.** A new boundary instrument costs a `POL-001`
§Verification amendment, which is tier 2 by construction — and `CLAUDE.md`
forbids compressing the instruments into a single count, so the amendment must
name what the new one holds and what it does not reach. A slice taking one
should consider taking the others in the same amendment.

### FU-7 — stratum 3 carries no manifest allowlist row
**Raised by** 005 (OQ-3, F-3) · **cost** tier 2 by construction · **scheduled as slice 012**

`crates/goad` has been unbilled since 002 and `crates/goad-emit` joined it.
`allowlist.rs`'s own module doc states the consequence so it is not
rediscovered: **a stratum-3 manifest is billed by nothing there.** The
instrument's subjects are exactly the two strata whose value is what they
*cannot* reach, which is how POL-001 §Verification scopes it, so
`crates/goad-emit`'s freedom from the renderer is held by the crate edge and by
review rather than by this file. Extending it to stratum 3 amends POL-001 rather
than applying it — the module doc says that too. 005 named *after 006* as the earliest sensible point,
when daily use had shown whether stratum 3 drifts at all; 006 is closed.

**Dead when** `allowlist.rs` bills a stratum-3 manifest and `POL-001`
§Verification states what that adds.

### FU-8 — no instrument holds *every write to a guarded widget goes through the counter*
**Raised by** 009 (`F-S5`) · **cost** tier 2 — a fifth boundary instrument, a markup scan

Hoisting any guard's assignment out of its comparison leaves every `-p goad`
target green with `reasserts` at `0` — every widget written on every present,
the caret destroyed on every tray check, and no tier observing it. The
production markup carries the counters deliberately and routes each write
through a counting function so the two are inseparable; nothing checks that the
next control added does the same.

**Dead when** a markup scan holds the routing, and `POL-001` §Verification says
what it reaches.

### FU-9 — the feature-unification residue is checked by nothing
**Raised by** 002 (D25) · **cost** tier 2 if landed as an instrument

A feature switched on elsewhere in the workspace can unify into stratum 1's
build with no instrument seeing it. One feature wide and harmless today
(`serde_core`'s `alloc`). A `cargo tree -e features` check would be the
instrument, if the residue ever grows enough to be worth automating.

**Dead when** either the residue is measured as wider than one harmless
feature, or a check holds it.

### FU-10 — citation discipline is enforced by nothing
**Raised by** 003, 004, 007, 009 (#8) · **cost** the instrument is tier 2; the sweep is tier 1 with a judgement per site

Two halves, one cause. **The rule**:
`docs/memory/cite-requirements-not-finding-ids.md` was settled at 001's audit
and closes with *"Do not extend the practice."* Slice 001's own code is exempt
by that decision; nothing else is. **Measured at `3ecaa11`: `F-N` appears 335
times across 54 files.** 004 named 37 sites in six files and the class has
grown well past them. Separately, nothing resolves a `docs/memory/` citation —
`path-flake-ref-breaks-on-demo-socket.md` was cited by a plan and six phase
briefs while not existing. And `CLAUDE.md`'s *cite by symbol* is held by no
gate step: the class regenerated **twice inside the commits that repaired it**,
and both times a script caught what two careful readings had missed.

The remedy is a judgement per site, not a rename: a comment explaining a
**requirement** cites `SPEC-NNN/R-N` or a §; one explaining a **measurement or
a runtime quirk** belongs in `docs/memory/` and is cited by file name.

**Dead when** a gate instrument resolves every in-repo `file.rs:NNN` and every
`docs/memory/` path in a comment, and the `F-N` census outside slice 001's code
is zero.

### FU-11 — properties held by nothing, needing a harness rather than a case
**Raised by** 003, 009 (#4, #10) · **cost** tier 1 each, but each needs something a unit cannot do

Four, sharing a shape — real, correct, and asserted by nothing:

- **`today_local`'s system zone.** `instant.rs` reads the system zone to seed
  an unpicked `datetime` picker; that the zone is the **system's** is held by
  review alone. The class is covered by
  `one_instant_is_two_different_local_dates_in_two_different_zones`; the token
  is not, because a unit cannot change the zone the machine is in.
- **The `enabled: !root.busy` bindings.** Some are held by the two cases 009's
  audit added; the rest are held by nothing, and removing them leaves every
  target green. Not a defect — the bindings are present and correct, and
  production disables every control. **009 #4 states a count that is already
  wrong**; count them at the grep, not from the entry.
- **`FLOOR_MILLIS`** in the renderer test tier is a hand-copy of the private
  `controller::MINIMUM_SPACING`, so changing the floor leaves the compile-time
  assertion that protects the anti-spin bound passing against a stale number.
- **The break-and-revert criteria are one-shot experiments.** Each established
  that an instrument fails when the thing it holds is removed, and each was
  reverted. The *necessity* argument lives in prose — `SPEC-002` §7 R-4 and
  ADR-004 say so rather than implying a test holds it.

**Dead when** each has an instrument, or is struck with the reason it cannot
have one.

### FU-12 — the production arrangement is unproven
**Raised by** 002 (F-5, F-8), 003 · **cost** tier 1; one part may never close here

Three parts, ordered by what is reachable:

- **One exchange through the production runtime topology.**
  `tests/event_loop/closing.rs` instantiates the real arrangement — Slint's
  executor polling a `tokio::process` future under an `EnterGuard` — and never
  drives an exchange through it. The panic 002's research measured has no
  regression test.
- **AC-9's last hop.** *A degraded body is still shown* is held at the
  `Presentation` boundary and argued, not asserted at the glass: the testing
  backend gives a `StyledText` element no content accessor. Either an upstream
  accessor lands, or a renderer-side probe is judged worth its cost.
- **The platform itself.** There is no headless way to install the production
  Slint platform, so every test runs on the testing backend's. **Not closable in
  this repository**, and it should not be quietly re-claimed as covered by a
  future slice.

**Dead when** the first has a regression test; the second when an accessor or a
probe lands; the third only from outside the test suite.

### FU-13 — `just check` goes red for load, not for a defect
**Raised by** 004, 009 (#5) · **cost** tier 1, but it is harness work

`POL-001` says the gate exits 0. Four standing flakes predate 004 and all wait
on a real subprocess and a real timeout —
`failure_matrix::a_backend_that_never_answers_reaches_the_caller_as_a_timeout`
and `transport::a_stdout_flood_is_refused_and_the_backend_sees_the_stream_close`
the two that reproduce readily, plus two more. Separately, 009's three loop
targets fail at roughly **6x** CPU oversubscription, and the failure is the
**liveness backstop** rather than any assertion: the slint-timer stepper stalls
for tens of seconds and `LIVENESS_BOUND` turns the stall into a red that reads
like a defect. A 40x nominal margin was not enough, so widening the bounds is
not the repair — the harness is. The reproduction numbers are in 004's
`notes.md` under PHASE-03 so this does not start from zero.

**Dead when** the gate's timed targets are insensitive to load, or the flakes
are diagnosed the way 004 diagnosed the reclaim case.

### FU-14 — `Refusal::reason()` is a convention, not a type
**Raised by** 004 (F-5) · **cost** tier 1, but a public API change

`reason()` returns `&'static str`, so nothing structurally prevents a new
token. The closure is held by an exhaustive `match` in the test file plus
review, and R-14's Verification row says so in terms. A closed `Reason` type
would make a new `Refusal` variant unable to mint a token at all — its forced
arm would have to name an existing `Reason`. Few surfaces: `reason()`'s
signature and its in-module `#[cfg(test)]` case in
`crates/goad-shell/src/ingress/mod.rs`, `folded()` in
`crates/goad/src/controller.rs`, and `Seen::Refused`'s payload type and the
closure case in `crates/goad-shell/tests/integration/ingress.rs`. The renderer
tier reads reasons off the reply JSON rather than off `reason()`, so it is
untouched — every `reason(...)` in `crates/goad/tests/renderer/ingress.rs` is
that file's own reply parser, which is why it looks like a caller list and is
not one.

**Dead when** `reason()` returns a closed type.

### FU-15 — whether ingress should serve connections concurrently
**Raised by** 004 (F-10) · **cost** tier 2 — `SPEC-003` §6.4 now states the property

The accept loop is sequential, so the per-read bound is also the longest one
connection can deny every other. §6.4 states that; whether the host should stop
having it is what is open.

**Dead when** §6.4 either keeps the property with a reason or loses it.

### FU-16 — three states no review round has reached
**Raised by** 007 · **cost** tier 1; budget, not tooling

All three are expressible with today's harness. Nothing has forced a genuine
`Full` on the one-slot channel in a running `serve`; nothing has killed a
backend mid-form with a draft outstanding; nothing has driven a real ingress
arrival at a window holding a half-filled form. The second is 007's own state;
the other two are 003's and 004's mechanisms meeting it.

**Dead when** each has a case.

---

## Band 4 — daily friction

Nothing is at risk. Someone is mildly worse off every day.

### FU-17 — the standing schedule is shown in raw UTC
**Raised by** 008 · **cost** tier 1

To a user on +10:00. Reading it costs a mental subtraction, every time.

**Dead when** the line reads in the zone the person is in, or says which zone it
is in.

### FU-18 — `goad-emit` takes no `--config PATH`
**Raised by** 005 (F-6) · **cost** tier 1

The host takes a configuration path as a positional argument; emit takes only
`--socket`, so `--socket` is the only route to a non-default configuration.
**005 named 006 as where this becomes obvious or stops mattering, and 006 is
closed** — the default path is the real one now, so the question is answerable
rather than waiting.

**Dead when** emit takes a configuration path, or the entry is struck with why
it never needed one.

### FU-19 — `next check (instructed)` is false whenever nothing was instructed
**Raised by** 009 (#7) · **cost** tier 2 — it carries a `SPEC-002` §6 question

The value is `now + default_poll` — R-26's third branch, which R-26
distinguishes from an instruction in as many words. A backend that never sends
`next_check` is legitimate, so for it the label is **always** wrong. Not a
relabel: `schedule.rs::resolve` returns a bare `Timestamp`, discarding the
branch at the moment it takes it, so making the line honest means `resolve`
reporting its branch through `State` to the diagnostics line — a change to a
pure stratum-1 function, its callers and its verification. What the line should
say for each of R-26's three branches belongs in `SPEC-002` §6.

**Dead when** the line names the branch it came from.

### FU-20 — the diagnostic surface cannot be selected or copied
**Raised by** 008 (L-4) · **cost** tier 1

The pane has had its visual pass; selection is the remainder, and it is the one
part that changes the element tree — a read-only `TextEdit` per line in place of
a `Text`, against a list `wiring.rs` selects by accessible label and item count.
Do it with those two selectors in hand.

**Dead when** a diagnostic line can be copied out.

### FU-21 — the diagnostic pane has no content-derived height
**Raised by** 008 · **cost** tier 1

L-1's cousin, left alone deliberately. The obvious binding is on the wrapped
height, which depends on the width just pinned to the scroller: a plausible
loop, and worth its own look rather than a guess.

**Dead when** the pane sizes to its content, or the loop is shown to be real and
the entry struck with what it costs instead.

### FU-22 — the idle surface is undecided
**Raised by** 008 (L-6) · **cost** tier 1; a behaviour question, not a layout one

With no view the surface is `Hidden` and the window with it, so there is nothing
to lay out. Candidates: leave it; put the next check somewhere a person passes;
keep a window up in an idle state. **Adjacent, and part of the same decision**:
`install.rs`'s `on_close_requested` quits the host, a deliberate earlier
decision, which makes the window something you cannot dismiss and get back.

**Dead when** the idle surface is decided, either way, in writing.

### FU-23 — magnification has two things open
**Raised by** 008 (L-9) · **cost** tier 1 · **unblocked 2026-09-23**

Persistence across a restart, deferred until the user has lived with it; and
whether the tray is the shipping affordance or scaffolding towards `Ctrl +/-`.
**The second was gated on 007's keyboard-focus follow-up, which 009 closed** —
a present that changes nothing no longer destroys the element that holds focus.
`app.slint`'s zoom comment still cites that follow-up as live and is now wrong.
The costing is in `docs/memory/window-zoom-is-the-scale-factor-event.md`.

**Dead when** both are decided.

### FU-24 — the void below the form under a tiling compositor
**Raised by** 008 (L-8) · **cost** tier 1

Stable and coherent now rather than scattered, but not designed. The same space
FU-22 might occupy.

**Dead when** it is designed, or FU-22 takes it.

### FU-25 — a slider reports its full unrounded value
**Raised by** 009 (#2) · **cost** tier 2 — it makes `step` normative for the value

Snapping the reported value to `step` was declined as a non-goal; the readout
landed instead, so screen and wire agree by construction. It displays the full
unrounded spelling, which is ugly — and that ugliness is the argument for this
in a form a person can see rather than one that has to be explained.

**Dead when** `step` is normative for the value, or the readout is judged
enough and the entry struck.

### FU-26 — the tray icon has no re-assertion path
**Raised by** 009 (#6) · **cost** tier 1

`glass.rs` calls `set_image` on every present, but `tray_icon` returns a
stable-address clone and slint's `ChangeTracker` fires only on `!=` — so nothing
re-registers an icon the platform has dropped. Raised as a follow-up and **not**
as a finding, deliberately: the non-recovery mechanism is confirmed, the cause
of the disappearance is not, and the witness was hedged. The durable part is the
class — *a repair that removes a redundant write also removes the self-healing
that redundancy was accidentally providing*.

**Dead when** a dropped icon is re-registered, or the disappearance has a cause.

### FU-27 — `goad-emit` has no `--timeout`, and no deadline at all
**Raised by** 005 (F-4) · **cost** tier 1

`SPEC-003` §6.4 makes the host's wait for judgement unbounded *by contract*: a
host not making progress holds the connection, and the writer waits with it
*"rather than being told something untrue"*. Emit therefore blocks indefinitely
and callers wrap it with `timeout(1)`. **005 named 007 as the evidence point and
007 passed without it.** If real use finds the blocking wrong, the flag exits 2
on expiry with a message saying the host may still act on the envelope.

**Dead when** use says the blocking is wrong and the flag lands, or use says it
is right and the entry is struck.

### FU-28 — `extraConfig`'s type admits what the module forbids
**Raised by** 006 (F-5) · **cost** tier 1

`nix/module.nix` declares `lib.types.attrsOf lib.types.anything`, which accepts
a nested attribute set and merges it straight into `Service`: `extraConfig = {
Unit = { … }; }` renders as `Service.Unit` and is refused by home-manager's own
type checker in the consumer's tree, one repository away from the module that
documents the restriction. A tighter type — `attrsOf (oneOf [bool int str
(listOf str)])`, or home-manager's own `unitOption` if it can be reached without
taking a home-manager input — would refuse it at the site that states the rule.
Deferred rather than fixed because which shapes are legitimate is a judgement
about the option surface, and the wrong narrowing costs a consumer a directive
they were entitled to.

**Dead when** the declared type refuses what the description forbids.

### FU-29 — a documented, verified non-nix build path
**Raised by** 006 · **cost** tier 1

It is a design goal that goad runs on non-NixOS systems, and 006's `design.md`
states that path is plain `cargo install --path crates/goad --locked`, needing
neither the wrapper nor `~/.config/goad/env` — but nothing outside that design
says so and nothing checks it. The scope: where the statement lives for a reader
who is not holding that design, and whether anything verifies it.

**Dead when** the statement has a home outside a closed slice's design, and
something checks it.

### FU-30 — a deliberate `nix flake update` is owed
**Raised by** 006 · **cost** work, not a slice

An update of every input was made on 2026-09-21 and deliberately reverted during
PHASE-05, so the cutover measured the artefact the phase gate had verified.
Whenever it is wanted: `nix flake update`, then `just check` and `just package`,
then `nix flake update goad` in `~/flakes` to carry it across. **Its own commit
is the point** — an input advance arriving inside another change is
indistinguishable from that change.

**Dead when** the update lands in a commit of its own.

---

## Band 5 — conditional

A trigger rather than a position. None of these is waiting on capacity.

### FU-31 — the protocol's open questions
**Raised by** 001, 003, 004, 007, 009 · **cost** tier 2 by definition

Four, and their reasoning is in `docs/roadmap.md` §Open decisions rather than
here, because each is a decision with an argument rather than a task with a
price. Named here so a sweep does not miss them:

- **`SPEC-001` OQ-1 and OQ-2** — capability declaration, and validation feedback
  with prefill. 007 sharpened OQ-2's trigger: it fires when use says a form must
  **reject** an answer.
- **`SPEC-001` OQ-4** — a date without a time. 009 changed the reason it is
  open, from an inexpressibility to an affordance cost, and the fork is
  asymmetric because R-18 already lets a renderer branch on a hint.
- **`SPEC-002` OQ-4** — a firing superseding a view a person is mid-answering.
  009 removed half the reason it stayed open.
- **`SPEC-001` OQ-3** — whether a stale `view_id` survives a host restart.
  Reopens with persistence; nothing persists today.

**Dead when** each is answered in its spec.

### FU-32 — the vocabulary scan reads lines
**Raised by** 001, 002 (D-13), 003 · **cost** tier 1

One instrument, three slices agreeing. A Rust string literal spanning lines
hides a `//` after the break; a `/* */` block spanning lines cuts line one and
scans the next as code; `r"` is recognised in `.slint`, where the construct does
not exist, which is inert; an all-caps compound has no case boundary to split
on; and path tokens match as substrings. Each is named in
`crates/goad-boundary/src/scan.rs` rather than assumed away. **003's
`scan::code_without_literals` is a new instrument beside `code_of`, not a repair
of these** — it holds a scan that counts *structure*, and these belong to the
scan that reads *words*. Latent: the build gate holds the stratum property
independently.

**Dead when** `src/` acquires one of those forms — that is the condition for
acting, not for closing.

### FU-33 — time-of-day strings no author writes on purpose
**Raised by** 001 · **cost** tier 1

`1:2:3:4:5` and `99:99` are *"a time of day"*, `T1:30` is unparseable where
`T18:00` is a time of day, and a config `timeout` written as a full datetime is
told it is a time of day. Recorded, not acted on.

**Dead when** any of them is reported by a real backend author; a fixture per
case then.

### FU-34 — a scheme policy for links inside rendered bodies
**Raised by** 002 (OQ-6) · **cost** tier 2 if it reaches the protocol

No URL is ever opened today, so it does not arise in practice — and no policy
has been decided either.

**Dead when** anything opens a URL, or a policy says nothing will.

### FU-35 — `BackendError::PipeMissing` and `cleanup_only` are reachable by no test
**Raised by** 001 (F-15, tolerated) · **cost** tier 1 · **carried to slice 013**

Either a unit test that fabricates the state, or removing the variant, when the
transport is reworked.

**Dead when** the transport slice takes one of the two.

### FU-36 — no end-to-end case for a silent backend
**Raised by** 001 · **cost** tier 1 · **carried to slice 013**

Nor for brief §10.1/§10.2 through a real process. Both are held at other tiers
through the one read site, and tolerated at audit. Cheap to add with the
instructed backend if the transport slice rebuilds the failure matrix.

**Dead when** that slice rebuilds the matrix, or declines to.

### FU-37 — `ConfigError::Read` names no subject
**Raised by** 006 (OQ-3, D1) · **cost** tier 1

Its text still says *configuration could not be read* and names no file.
Deliberate: a path inside it would print twice in `goad-emit`, which renders the
path itself. Nothing at stratum 3 renders that string any more, so it is latent
rather than live. The question: whether a stratum 2 error that names no subject
should carry that wording at all.

**Dead when** anything at stratum 3 renders it again, or the wording is settled.

### FU-38 — duplicated `#[cfg(test)]` view fixtures
**Raised by** 009 (`notes.md` §Open, never promoted to §Follow-ups) · **cost** tier 1

The fixtures that read an id off a normalized view are duplicated between
`draft.rs` and `view_model.rs`, because neither module can reach the other's
test module. A `#[cfg(test)] pub(crate) mod fixtures` under `lib.rs` is the
obvious home; it is a decision rather than a repair, because `lib.rs`'s own doc
says it is the module tree *and nothing else*.

**Dead when** a third module wants the same fixture — that is when it must be
answered rather than copied.

---

## Methodology and record

Not slices. Each is an edit to how work is done here, or to a document about it.

### FU-39 — a phase cannot check its own Surfaces line
**Raised by** 007 · **cost** a `docs/AGENTS.md` change

Five of six were short, because a sheet is written before the work and nothing
re-reads it after; the audit caught it four times. Either the phase reconciles
its Surfaces line at exit, or the per-phase enumeration is dropped in favour of
the slice-level §Scope that no phase breached.

**Dead when** `AGENTS.md` §Execute or §Phase plan says which.

### FU-40 — `CLAUDE.md`'s authoritative-documents table names only SPEC-001
**Raised by** 003 · **cost** a one-line edit, and `CLAUDE.md` is amended with endorsement

SPEC-002 and SPEC-003 now exist beside it. 003's reconciliation did not amend
the row because the endorsement it held covered a different sentence, and its
`audit.md` records the omission as deliberate.

**Dead when** the row names the specs that exist.

### FU-41 — slice 001's design drift is left as written
**Raised by** 001 · **cost** none; it is a standing decision

By user decision: SPEC-001 is the living truth, and each departure is documented
at its site in the code. Listed so a sweep does not read it as an omission.

**Dead when** never — struck only if the decision is reversed.

---

## Closed

Struck, not deleted, so a reader who remembers one finds what killed it.

**Swept 2026-09-23 at `3ecaa11`:**

| raised by | claim | what killed it |
|---|---|---|
| 001 | ~~the timer must not retry a failed exchange faster than the resolved instant, and must not re-resolve on its own~~ | slice 003 built the timer; its own §Follow-ups sweep does not carry it forward |
| 001 | ~~a discarded `next_check`'s `raw` renders verbatim and unbounded, newlines included~~ | every line composed in `Diagnostics::of` passes `finish(.., LINE_LIMIT)` |
| 005 | ~~ADR-003 §Decision says *four members*; there are five~~ | ADR-003 reads *"four at this decision, five since slice 005"* |
| 007 | ~~the window has no content-derived preferred size — 50×65 regardless of content~~ | 008: `PromptWindow` declares `preferred-width`, `min-width`, and a `min-height` taken off the stack's preferred height |
| 007 | ~~the title and body are clipped at the top~~ | the same repair |
| 007 | ~~the diagnostic surface wraps no line~~ | 008 pins the line layout to the scroller's visible width and turns the horizontal bar off |
| 007 | ~~a captured stderr line's trailing newline renders as a visible `\n`~~ | `without_one_terminator` |
| 007 | ~~keyboard focus does not survive a present~~ | 009: a present now writes through two channels behind a guard, so one that changes nothing destroys nothing. **`app.slint`'s zoom comment still cites this as live and is wrong**; it unblocks FU-23 |
| 007 | ~~nothing drives `choice`, `number` or `datetime` through `serve` to a `respond`~~ | 009 verified all five kinds against the child process's own request log |
| 007 | ~~`docs/roadmap.md` names `Undrawn::OptionFields`, which is gone~~ | the name is no longer in `roadmap.md`; `slice-007.md` keeps it as a statement of the problem 007 opened on |
| 007 | ~~D3's rationale has no home outside this slice~~ | it is in `docs/roadmap.md` §Open decisions |
| 007 | ~~socket transport: promote or defer, now decidable~~ | **answered, not killed.** Deferred. 007's own form collapsed fourteen spawns per slot into one, so the measurement 007 was waiting on says the transport is less urgent than when it was raised. It moved down again rather than up; `roadmap.md` §Sequence records it |

**Two count-rot instances found in the same sweep**, repaired where this file
could reach them: `roadmap.md` said 009 produced *nine* follow-ups against a
table of ten, and 009 #4 states a count of `enabled: !root.busy` bindings that
the markup has already outgrown. Both are the class `CLAUDE.md` §Working here
names. Neither is a follow-up; both are why *name, never count* binds this file
too.
