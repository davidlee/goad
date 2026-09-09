# Audit & reconciliation — Slice 004

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `b6ca5f7..93abab3` on `main` — the eight phases. `b6ca5f7` opened
the slice; the code phases begin at `9cfb679`; `93abab3` ends them, and all
eight phases are `done`.

**Refreshed 2026-09-09 (`review-code.md` F-16).** `93abab3` is no longer HEAD
and this document was arguing for a tree that is not the one shipping. The
shipping tree adds a second range, `93abab3..HEAD`, holding the repairs made for
`review-code.md` rounds 1 and 2 and the ledger and document corrections that go
with them. **The two ranges were made under different disciplines and are kept
apart everywhere below rather than merged:** the first is phase work with
declared surfaces, the second is repair work answering a numbered finding.
Everything in the original range stands as audited; every correction made since
is dated and says which finding drove it.

**Question:** slice 004 is finished when a watcher outside the host can make it
ask its backend something, on the path slice 003 built, without the host having
learned what the event meant — and when the record says so truthfully. This
audit checks the following, and says plainly where the evidence does not carry.

**Lines of attack, chosen before looking:**

1. **The gate, run rather than cited.** `just check` executed here, transcript
   saved, the real exit code reported. A phase that reports green is a claim;
   the gate is the evidence. Slices 001–003 all closed green on a binary that
   could not open a window, so a green gate is the floor and not the argument.
2. **Each of AC-1..AC-13 against a named test function or a code citation.**
   `plan.md`'s AC-to-phase table is a map to check, not the answer: the question
   is whether an assertion exists that would fail if the criterion were false,
   not whether a phase claimed the criterion. Four ACs took readings during
   design (AC-1's *verbatim*, AC-3's *every envelope*, AC-6's *anchors*, AC-9's
   *exit code*); each is held to the reading as written, not to a looser one.
   AC-9's exit-code clause is held by review by construction — the audit checks
   the argument at `main.rs`, and checks that no instrument was quietly claimed
   in its place.
3. **Each VT/VA/VH in `plan.md`, systematically.** Discharged or not, with the
   test that discharges it. A verification criterion with no assertion behind it
   is the same defect as an unmet AC wearing different clothes.
4. **The surface delta, walked from `git log` myself.** PHASE-07's sweep already
   claims 26 files and no undeclared paths; an audit that reads a sweep's
   conclusion has audited the sweep. Undeclared paths are the highest-signal
   lead; declared-but-untouched is checked in the other direction.
5. **The five invariants, as invariants and not as slogans.**
   - *The host does not understand the domain.* The envelope is carried, not
     read. `source`, `kind` and `data` reach the backend unexamined; no new host
     type or module name carries domain vocabulary; the scan passes because
     there is nothing to find, not because the scan is narrow.
   - *Permissive wire, canonical internals.* The envelope's normalization is the
     only door, and past it nothing is unvalidated. An ambiguous envelope is
     refused rather than guessed at.
   - *Wire compatibility is not narrowed to the renderer.* The ingress contract
     is what `draft-spec.md` states, not what the current loop happens to
     consume.
   - *A backend — or here, a writer — failure never takes the host down*, and
     never leaves it unable to invoke the backend again. AC-12 is the test of
     it; the audit also looks for the paths where a panic or an early return
     could reach the loop.
   - *Strata run one way.* Stratum 1 gains no dependency, `src/semantics/`
     names no shell, and the envelope's normalization sitting in stratum 2 is
     the deliberate ADR-001 §Consequences call the slice says it is — the audit
     checks that the new ADR is actually owed and named, not assumed written.
6. **Canon and drafts, as obligations rather than as done work.** CD-1, CD-2,
   CD-3 and the new ADR are unapplied by design until reconciliation; the audit
   records what endorsement each needs and writes the Reconciliation rows
   unchecked. `SPEC-002/R-12` and `SPEC-003/R-12` are two different
   requirements: any citation that drops the prefix is a defect.
7. **AC-13, the human observation.** Lifted from `notes.md`, with an honest
   account of which runbook steps a person actually performed.

**What this audit does not do.** It does not write the code review — that is
`review-code.md`, a fresh adversarial agent, after this. It does not amend
canon, promote `draft-spec.md`, or apply `canon-delta.md`: those are user gates.
It does not repair findings. It records, disposition-ready, and stops. The
Verdict is written from the evidence and says so where the evidence runs out;
the Closure checklist stays unticked while the code review has not run.

## Evidence

### Tests and checks

**On `93abab3`, the phase-end tree.** `just check` run here, with a clean tree,
under `nix develop`. **Exit 0.** Transcript:
`/tmp/claude-1000/-home-david-dev-goad/d6c2f7fb-4990-41a6-8864-c625c53be6f8/scratchpad/just-check.log`
(session-local). All six commands present, in the order
`docs/policy/001-the-phase-gate.md` states: `cargo build --workspace`,
`cargo test --workspace`, `cargo test -p goad-semantics`, `deno check
examples/typescript/backend.ts`, `cargo clippy --workspace --all-targets -- -D
warnings`, `cargo fmt --all --check`. Nineteen `test result: ok` blocks, zero
`FAILED`, zero warnings. 368 tests in the workspace run and 35 more in
stratum 1's own run.

This is the gate as evidence rather than as a claim; it is also the floor and
not the argument (`docs/AGENTS.md` §Tiers, and slices 001–003).

**On the shipping tree — added 2026-09-09 (`review-code.md` F-16).** `just
check` re-run after the round-1 and round-2 repairs. Two runs on two trees is a
fact worth recording rather than collapsing into one, and the second run is not
uniformly green:

- The reviewer re-ran it at `441fa94`: **exit 0**, 19 `test result: ok`, zero
  failures, recorded in this ledger's round-2 synthesis.
- Run again here at `aef04c4`, the round-2 repair tree, `just check` **exited 0
  on all but one of several runs**; and three consecutive times at `4fb79c0`,
  the tree this refresh is dated to, exit 0 each time. The single failure was
  `ingress::a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`,
  refused `in use by a live host` — **the reclaim flake `99abac4` chased and
  could not reproduce.** It is now reproduced, and it is **not** the repairs':
  measured at 1 failure in 35 sequential runs of the integration target on the
  shipping tree and **2 in 60 on a worktree at `93abab3`**, which predates every
  repair. Same case, same message, comparable rate. Recorded in `slice-004.md`
  Follow-ups with the conditions, since that entry exists so a future slice does
  not start from zero.

**The honest reading:** the gate passes on the shipping tree and is not
deterministic on either tree, for a reason that predates the slice's repairs and
is already owned as a follow-up. A green run is evidence; it is not proof the
flake is gone.

**Standing caveat, not this slice's.** `slice-004.md` Follow-ups records four
pre-existing flaky tests, all waiting on a real subprocess and a real timeout,
reproduced under load by PHASE-03. The reclaim case above is a fifth, and the
one that had resisted reproduction until now.

### Acceptance criteria

Every criterion is discharged. The evidence is a test function that would fail
if the criterion were false, or — for AC-9's exit-code clause alone — the
argument the slice declared in advance would hold it.

| AC | met | evidence |
|---|---|---|
| AC-1 | yes | `renderer/ingress.rs::a_well_formed_envelope_produces_one_evaluation_carrying_all_four_fields` — one `evaluate`; `source`, `kind` and `data` asserted against the bytes written; `timestamp` asserted as the **same instant** (`+10:00` in, `Z` out, compared as parsed `jiff::Timestamp`s), which is the reading `slice-004.md` took; `now` asserted to be the host's stub clock and not the envelope's |
| AC-2 | yes | `renderer/ingress.rs::the_view_an_ingested_evaluation_returns_reaches_the_window_and_is_answerable` — the token is read off the options model with `current_view_token`, the same helper the scheduled tests use, and answered |
| AC-3 | yes | `goad-shell/tests/integration/ingress.rs::an_envelope_terminated_by_a_newline_is_accepted`, `…_by_closing_the_write_side_is_accepted`, `a_second_envelope_on_the_same_connection_is_never_read` (exactly one reply line, then EOF, one arrival at the judge), `a_dropped_answer_yields_unavailable_then_a_close`. **Corrected 2026-09-09** (`review-code.md` F-8, whose Response promised this amendment and whose repair pass did not make it — caught by F-16): this row previously argued that *"the one admitted exception is visible in the code: `ingress/mod.rs:487-489` returns without replying only when `arrivals.send` fails — the judge is gone, which is the host process going away."* Every clause of that is now false. **There is no admitted exception on that path**: `handle` writes `Unavailable(Stopping)` and closes before returning, so a connection accepted after the judge is gone is answered like any other, and `a_connection_accepted_after_the_judge_is_gone_is_answered_unavailable` asserts the reply the old row said did not exist. The justification was also wrong on its own terms — F-8 showed *the judge is gone, which is the host process going away* to be a timing assumption rather than a property, which is why the code changed rather than the reading. R-8's one admitted exception is now reached only when the process itself is gone |
| AC-4 | yes | closed set asserted as a set: `the_reason_token_set_is_closed_at_eight` compares `Refusal::reason()` over all eight constructions against a literal list, so a token **removed or renamed** fails there, and the eight-way mapping with it. **Corrected 2026-09-09** (`review-code.md` F-5): this row previously said *"added, removed or renamed"*. A token *added* is not held by that assertion — it is held by the case's exhaustive `match`, which fails to compile when a variant is added, plus the review that follows. `draft-spec.md` R-14's Verification row was weakened to match rather than the claim being kept. Each reason read off the wire: `the_three_shape_reasons_this_phase_owns_are_read_off_the_wire` (`malformed`, `invalid_envelope`, `reserved_source`), `more_than_the_byte_limit_…` (`too_large`), `a_connection_that_writes_nothing_times_out_…` (`timed_out`), `renderer/ingress.rs::an_envelope_arriving_during_an_exchange_is_refused_engaged_before_it_completes` (`engaged`), `…a_second_envelope_inside_the_spacing_is_refused_too_soon_and_says_how_long` (`too_soon`). All four reasons AC-4 names *at least* are among them |
| AC-5 | yes | `renderer/ingress.rs::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal` — the invocation count bounded, the excess replies naming `too_soon` (refused, not delayed), and the presentation cost fixed at one per refusal by `assert_eq!`. Measured: 845 refusals, 845 presentations, ~1690/s (`notes.md` PHASE-07) |
| AC-6 | yes, and falsifiably | three tests, one per crossing: `an_ingested_firing_never_writes_the_scheduled_floor` (i), `an_ingested_firing_does_not_advance_the_scheduled_floor` (ii — ADR-004's case), `a_scheduled_firing_does_not_clear_the_event_floor` (iii). (ii)'s setup pins **both** the T₀ exchange and the ingested exchange to the same one-second `next_check`, which is the clause `slice-004.md`'s reading calls load-bearing; without it the case would turn on a deadline both hypotheses agree about. Falsification is recorded, not assumed: PHASE-05/VA-3 broke the anchor three ways and each break turned the expected case red while the other two stayed green (`notes.md` PHASE-05) |
| AC-7 | yes | `config.rs::with_no_ingress_section_ingress_is_none`; `startup.rs::listener::none_binds_nothing` (a directory watched across the call gains no entry, paired with `some_path_binds`'s positive so the negative is not vacuous); and the existing suite with assertions unchanged. Checked directly here: across the five bounded files the whole diff is one `use goad_shell::ingress::Ingress;` per file, one `Ingress::none(),` or `ingress: None,` per call site, and rustfmt's reflow of the now-multiline `serve(…)` call. `renderer/scheduling.rs`'s 133 added lines are entirely that reflow. No assertion moves — AC-7's reading holds |
| AC-8 | yes | `a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves`; `a_live_socket_refuses_a_second_bind_and_keeps_serving`, which asserts the first listener still serves by writing an envelope to it and reading a reply rather than by inspecting the error alone |
| AC-9 | yes, in two parts | The error value and the message: `a_regular_file_at_the_path_is_refused_naming_what_was_found`, `a_directory_with_no_write_permission_is_refused_naming_the_path`, `startup.rs::listener::some_path_that_is_a_regular_file_names_the_path`, `display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path`, `stderr_outlets::report_startup_line_renders_ingress_like_its_siblings`. **The exit code is held by review**, as the slice declared before building: `main.rs:21-29` is a single `match run()` mapping every `Err` to `ExitCode::from(2)`, and it is **unchanged by this slice** — verified here against the diff, which touches `start` and nothing above it. No instrument was claimed in its place, and no binary-running harness was built |
| AC-10 | yes | `the_socket_is_owner_only_after_bind` asserts `mode() & 0o777 == 0o600` over the host's own `set_permissions`. Non-vacuous under every umask but `0o177`; PHASE-03 recorded the run's ambient umask as `0o022` |
| AC-11 | yes | Each instrument separately, run here as part of the gate. Crate-edge rule: `cargo build --workspace` resolves. Manifest allowlist: `allowlist::the_real_stratum_1_manifest_is_clean`, `…stratum_2…`. Stratum 1 purity: `purity::the_real_stratum_1_source_names_none_of_the_nine`. `cargo test -p goad-semantics`: 35 tests, green, built with its own feature set. Separately, the vocabulary scan: `vocabulary::no_workspace_member_names_the_users_domain` and `…_in_its_own_crate_name`, and the scan reads `workspace.members` rather than a hand-list (`vocabulary.rs:44-58`), so `crates/goad-shell/src/ingress/` is covered by construction. Stratum 1 gains no dependency: `crates/goad-semantics/Cargo.toml` has an **empty diff** across the whole slice, and `cargo tree -p goad-semantics -i tokio` matches no package, so `net`/`sync` do not reach stratum 1's graph. Its only source change is the two edits PHASE-02 declared — `pub(crate)` → `pub` and one word of a doc comment. No new module name or type name in the slice is on the `DOMAIN` list: `ingress`, `envelope`, `arrival`, `refusal`, `answer`, `event`, `source`, `kind`, `listener`, `watcher` |
| AC-12 | yes | `a_malformed_envelope_reaches_no_event_and_the_listener_stays_up` (no `Event` at the judge, and the listener answers the next envelope); `renderer/ingress.rs::after_a_flood_of_malformed_envelopes_the_host_still_evaluates` (zero invocations from the flood, one after it); `a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating` (the closed-channel path folds once, does not spin, and the host still evaluates). `Ingress::arrival` sets `arrivals = None` as it yields `None` and thereafter parks on `future::pending()` (`ingress/mod.rs:216-228`), which is what makes the no-spin assertion true rather than lucky |
| AC-13 | yes | The user's own run, 2026-09-09 — see **The human observation** below |

### The human observation (AC-13, PHASE-06/VH-1)

Run by the user on 2026-09-09 from the repo root in the dev shell, and recorded
in `notes.md` under `## VH-1 — the observation`. It is the user's account; the
executing agent did not run it and this audit does not corroborate it.

`just demo` opened a window on the demo backend's fixed prompt, visually
unchanged from before the slice. From a second shell the user emitted

```
^printf '{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-08-22T17:10:00+10:00","data":{}}' | socat - UNIX-CONNECT:./goad-demo.sock
```

(`^printf` because the user's shell is `nu`; `data` empty rather than carrying
`count_last_hour` — `backend.sh`'s catch-all arm reads `source` and `kind`
only, so the same path is exercised). The emitting shell received
`{"protocol":1,"accepted":true}` on the same connection, and **the window's
title became `An event arrived: reddit-watcher / reddit-opened`**.

That is AC-13: a person started goad with ingress configured, wrote an envelope
from the documented one-liner, and watched a prompt naming their own event
appear — over a path no test reaches, because no test opens a window. It is
consistent with the code: `examples/shell/backend.sh`'s new catch-all arm
composes exactly that title, and its `"source":"host"` arm is why step 1's
startup view was unchanged.

**Runbook steps 4 and 5 were not run by hand.** The repeat-emit inside the
spacing and the malformed emit were not exercised by a person. Both are covered
by automated tests — the spacing by PHASE-04's arms, the malformed by
PHASE-03's — and neither is what VH-1 exists to witness, so their absence does
not qualify the discharge. It is recorded because the runbook has five steps
and three were run. Nothing else was reported: no hang, no leftover socket, no
unexpected refusal. Absence of a report is not a positive check, and PHASE-06's
S-3 and S-4 are recorded as not fired on that basis only.

### Verification criteria

Every VT, VA and VH in `plan.md` is discharged, and every one names a test that
exists in the tree at the name claimed. Checked by reading the function lists
out of the files rather than by reading the phase sheets' claims.

**PHASE-01 — the A-1 probe.** VT-1..VT-4 (P-A..P-D) each run three times, output
pasted into `research.md` Thread 3; verdict *A-1 holds*. VA-1 green with the
manifest reverted — confirmed here: `crates/goad/Cargo.toml` has an **empty
diff** across the whole slice, so EX-3's revert took.

**PHASE-02 — the config key and the envelope.** VT-1 (`an_ingress_section_loads_with_its_path`,
`an_empty_ingress_path_is_refused`, `an_unknown_key_inside_ingress_is_refused_and_named`),
VT-2..VT-9 and VT-11 in `envelope.rs`'s own test module, VT-10
(`with_no_ingress_section_ingress_is_none`). VA-3 re-checked here: `git diff
crates/goad-semantics/` is exactly the two edits declared — the visibility of
`json_type_name` and one word of its doc comment — and nothing else. VA-4
re-checked: one added field per bounded file.

**PHASE-03 — the socket, the read, the accepted path.** VT-1..VT-6 and
VT-10..VT-14 all present as named tests in
`crates/goad-shell/tests/integration/ingress.rs`. VA-2's margin: VT-14 at
501.87 / 501.71 / 501.82 ms against a 500 ms `ENVELOPE_DEADLINE` — recorded as
**exempt**, the wait being the bound under test. VA-3: no socket left in the
checkout.

**PHASE-08 — the refusal vocabulary.** VT-7, VT-8, VT-9 (two functions) present.
VA-5 re-checked here: `crates/goad-shell/Cargo.toml`'s only change in the whole
slice is PHASE-03/EX-1's one line, so PHASE-08 added nothing to the manifest.

**PHASE-04 — `serve`'s two arms and the second anchor.** VT-1..VT-7 present in
`renderer/ingress.rs`; VT-8 is
`controller.rs::tests::a_writer_arriving_exactly_at_the_anchor_is_outside_the_spacing`,
in the crate's own `#[cfg(test)] mod tests` beside `deadline_after`'s cases, as
planned — confirmed present. VA-3 (the bounded files are bounded) and VA-4
(break-and-revert on the second anchor: VT-4 red, VT-1..VT-3 green, reverted)
recorded in the sheet.

**PHASE-05 — the two anchors, and what a person can see.** VT-1..VT-6 present.
VA-3, the break-and-revert, is the strongest single piece of evidence in the
slice: three isolated breaks, each turning exactly the case that should go red
red — Break 1 → VT-1, Break 2 → VT-3, Break 3 → VT-2 — with the other two
staying green under each. AC-6's tests are falsifiable rather than merely
passing.

**PHASE-06 — binding at startup, and the demo.** VT-1 (`some_path_binds`, which
asserts a real socket file type and not merely `Ok`, and
`some_path_that_is_a_regular_file_names_the_path`), VT-2 (`none_binds_nothing`),
VT-3 (both outlets). VA-2 the clean clone; VA-3 the exit-code argument; VH-1
above. **EX-1's "`source()` arm" was discharged by the default rather than by an
added arm** — `StartupError`'s `impl Error` stays empty and
`startup_error_source_is_always_none` was extended to cover the ninth variant,
with the reason written into the test's doc comment. That is a reasonable
reading of a criterion phrased for a different implementation, and it is
recorded rather than passed over.

**PHASE-07 — the sweep and the gate.** VT-1 adds no test, as planned. VA-1's
clean-clone gate, VA-2's instrument-by-instrument AC-11 walk, VA-3's surface
sweep and VA-4's vocabulary re-read are recorded. This audit re-ran VA-2's
substance and VA-3's whole walk independently; see below.

### Surface delta — walked here, from the git history

**This walk covers `9cfb679^..93abab3` — the phase range — and nothing after
it.** The repair range is walked separately below, because its files arrived
under a different discipline and retro-fitting them onto a phase's *Surfaces*
line would make this walk say something untrue about what the phases declared
(added 2026-09-09, `review-code.md` F-16).

Taken from `git diff --name-status 9cfb679^..93abab3`, not from PHASE-07's
sweep. **26 non-documentation files changed.** Every one is inside a surface
some phase declared, with a single exception:

- **`Cargo.lock`** is named by no phase's *Surfaces* line. Its whole diff is
  eleven lines: `socket2` entering the graph as a transitive dependency of
  `tokio`'s `net` feature. That is the mechanical consequence of
  PHASE-03/EX-1's declared manifest change — a lockfile that had *not* moved
  would mean the manifest change did not take. Recorded as an undeclared but
  benign path, not as scope creep. It is the only file this audit's walk and
  PHASE-07's sweep both had to argue rather than match.

The rest map cleanly: `.gitignore`, `flake.nix`, `examples/demo.toml`,
`examples/shell/backend.sh`, `crates/goad/src/startup.rs` and
`crates/goad/tests/renderer/startup.rs` to PHASE-06; `crates/goad/src/main.rs`
to PHASE-04 (the call site) and PHASE-06 (the startup order);
`crates/goad/src/controller.rs`, `diagnostics.rs`, `tests/renderer/main.rs` and
`tests/renderer/ingress.rs` to PHASE-04, the last extended by PHASE-05;
`crates/goad-shell/src/{config.rs,error.rs,lib.rs}`,
`crates/goad-semantics/src/error.rs` and `tests/support/driving.rs` to PHASE-02;
`crates/goad-shell/src/ingress/{mod.rs,envelope.rs}` to PHASE-02/03/08;
`crates/goad-shell/Cargo.toml` and `tests/integration/{main.rs,ingress.rs}` to
PHASE-03 and PHASE-08; `crates/goad/tests/renderer/{scheduling.rs,wiring.rs}`
and `tests/event_loop{,_schedule}/…` to PHASE-02's bounded field and PHASE-04's
bounded argument.

**The bounded edits are bounded.** Checked by filtering the added
`Ingress::none(),` and `ingress: None,` lines out of the diff: what remains
across all five test files is one `use` line each and rustfmt's reflow of
`serve(…)` into multiple lines. No renamed symbol, no changed assertion.

### Surface delta — the repair range (added 2026-09-09, `review-code.md` F-16)

`git diff --name-status 93abab3..HEAD`, non-documentation files. **No phase
declared any of these, and none should be retro-fitted to one**: they are the
repairs `review-code.md` rounds 1 and 2 required, and the finding each answers
is the declaration. Six files:

| file | findings |
|---|---|
| `crates/goad-shell/src/ingress/mod.rs` | F-1, F-2, F-4, F-7, F-8 |
| `crates/goad-shell/tests/integration/ingress.rs` | F-1, F-5, F-8, F-11 |
| `crates/goad-shell/tests/integration/round_trip.rs` | F-12, F-14 |
| `crates/goad/src/controller.rs` | F-3, F-4, F-13 |
| `crates/goad/tests/renderer/ingress.rs` | F-3 |
| `examples/shell/backend.sh` | F-12, F-14 |

**`round_trip.rs` is the one the phase walk above does not list, and correctly
so** — it was not touched by any phase. It enters here because F-12 needed the
example backend driven through the real transport, and F-14 extended that case's
neighbour. Its provenance is a finding, not a phase; the walk says so rather
than omitting it, which is how it went unrecorded until F-16.

`Cargo.lock` does not move in this range: **no repair added a dependency**, and
two declined to — F-5's closure instrument and F-7's read-fault case both stop
at a stated boundary rather than take one (`review-code.md`, those findings'
Responses).

**Declared but untouched:** `crates/goad/tests/renderer/harness.rs`, which
PHASE-05 declared conditionally ("**only** if PL-8's counting glass gains a
second consumer here"). It did not, so the glass stayed in `ingress.rs`. Not
dropped work. `docs/slices/004/ingress-probe.local.rs` exists in the working
tree and is untracked by design (`.gitignore`'s `*.local.*`).

**No path was touched outside a declared surface, and canon was not touched at
all** — `git diff --name-only b6ca5f7..93abab3 -- docs/specs docs/policy
docs/adr` is empty, which is what the slice said would be true until
reconciliation.

### Known open items

- **F-1 — the ingress-stopped `unavailable` token is a literal.** Reported by
  PHASE-07 and deliberately left for the code review to disposition.
  `controller.rs:446-455`'s `ingress_stopped()` builds `Refused::Ingress {
  reason: "unavailable".to_owned(), … }` directly; the only other construction
  site, `refuse_arrival` at `controller.rs:418-424`, derives `reason` from
  `refusal.reason()`. Confirmed here by reading the file. It is not drift —
  `design.md` §5.2 specifies exactly this mechanism, and the cause answers no
  envelope so has no `Refusal` value to borrow from. The gap is that nothing
  fails if the literal is misspelled: `the_reason_token_set_is_closed_at_eight`
  closes `Refusal`'s set and this literal is not `Refusal`'s, and
  `a_dead_accept_task_is_folded_once_…` asserts on `detail` ("ingress has
  stopped"), not on `reason`. Open, undispositioned, for `review-code.md`.

- **`design.md` §10's CD-3 row calls the ADR-004 case "AC-6's *third* test".**
  It is the second. `design.md` §9's own AC-6 row, `canon-delta.md` CD-3
  ("the *advances* case, `design.md` §9 (ii)"), `plan.md`'s Coverage table and
  `slice-004.md`'s reading all name it **(ii)**, and the implemented test
  (`an_ingested_firing_does_not_advance_the_scheduled_floor`, PHASE-05/VT-2) is
  that case. Nothing downstream was built on the wrong test — the clause "the
  *advances* direction" in the same sentence fixes the referent — so this is a
  stale ordinal in one clause and not a defect in the work. Found by this audit;
  it is a Reconciliation row below, not a code finding.

### A lead handed to the code review, not dispositioned here

`accept_loop` (`crates/goad-shell/src/ingress/mod.rs:452-463`) `continue`s on
every `accept()` error without bound or backoff. A transient error is right to
retry; a persistent one — the listener's descriptor in a state that fails every
`accept` — would spin the accept task. It is on a spawned task rather than the
main thread, so it costs no presentation and AC-12's "never takes the host down"
is untouched, but it is the same shape as the spin PHASE-04/VT-7 exists to
exclude on the loop side, and no case drives it. Named here so the reviewer
meets it rather than finds it; the disposition is theirs.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md` — **not yet opened.** The adversarial code review
  is a fresh agent and has not run.
- **State:** open · outstanding blockers: unknown, the review has not run
- **Carried in for it:** F-1 (the ingress-stopped `unavailable` literal), and
  the `accept_loop` retry lead under Evidence. Neither is dispositioned here.

## Verdict

**On the evidence, the slice does what it set out to do.** Nothing found in this
pass argues against closure; the argument is not complete, because the code
review has not run.

Something outside goad can now make it ask its backend a question. A watcher
writes an envelope to a Unix socket, the host forwards its four fields
unexamined, and the backend decides what appears. A person has seen it happen:
the window's title named the `source` and `kind` they chose, over a path no test
reaches. That is the thing the slice existed to build, and it is built.

The parts that could have gone wrong quietly did not. The evaluation path is the
one slice 003 built rather than a parallel one — an arrival becomes a
`Pending::Evaluate` at the same join every command reaches, and `wire.rs` is
untouched. The two anchors are independent in all three directions, and that is
held by tests which have been *shown* to fail when the anchor is broken rather
than merely observed to pass. The host learned nothing about the domain: no new
type or module name carries a domain word, `data` is carried opaquely, and the
one stratum 1 change in the slice is a visibility keyword. A host with no
`[ingress]` key binds nothing and behaves exactly as before, with every existing
assertion unmoved.

Three things are being accepted knowingly.

**The refusal a real watcher will meet most often is invisible to the person
debugging that watcher.** `engaged` is decided during an exchange and is
overwritten by that exchange's own outcome before anything is presented. AC-3 is
untouched — the writer always gets its reply — but the diagnostics surface is
one whole value, and only refusals decided while idle survive to it. This is
`slice-004.md` Follow-ups, and it is the follow-up most likely to be felt.

**The socket has no owner after startup.** Nothing prevents two goad processes,
nothing re-probes the path once bound, and a socket unlinked underneath a live
listener leaves it holding a descriptor no `connect` can reach — with nothing to
report, because nothing arrives. The slice declined to close this inside the
ingress module because doing so would be a partial single-instance guarantee
under another name. That reasoning stands; the gap is real and is written down.

**Four pre-existing flaky tests remain the likeliest source of an unexplained
red gate.** They predate this slice and sit outside its surfaces. This audit's
gate run was clean, which retires nothing.

The Closure checklist is left unticked. Two of its items — the code review, and
canon reconciliation — have not happened, and one of those is a user gate.

## Reconciliation

Every row needs explicit user endorsement except the last two, which amend the
slice's own documents rather than canon. None is done.

| document | change | reason | done |
|----------|--------|--------|------|
| `docs/specs/002-host-scheduling-behaviour.md` | apply `canon-delta.md` **CD-1** — add **SPEC-002/R-12**, the event bound and the principle it instances; amend §2's count of what the spec abuts and §6's sentence naming what the host owns | R-5 required a host adding a stimulus other than a due check to decide separately how it is bounded. This slice decided it; the spec must now say so | [ ] |
| `docs/specs/001-host-backend-protocol.md` §R-56 | apply `canon-delta.md` **CD-2** — narrow the first clause to every `evaluate` the host originates *on its own account*, and add the clause reserving `"host"` as a source | without the narrowing a conforming host breaches R-56 the moment it forwards an ingested event. The matching *refusal* is SPEC-003's, not this spec's | [ ] |
| `docs/adr/004-scheduled-firings-are-spaced-from-the-previous-scheduled-firing.md` §Verification | apply `canon-delta.md` **CD-3** — name the three AC-6 tests by file and function, saying which discharges the debt; add `design.md` §5.3 and §9 to References. The decision itself is untouched | the ADR says the case that separates the anchor from the boolean is the one slice 004 will introduce. `renderer/ingress.rs::an_ingested_firing_does_not_advance_the_scheduled_floor` is that case, and it passes and has been shown to fail when the anchor is broken | [ ] |
| `docs/slices/004/draft-spec.md` → `docs/specs/003-host-event-ingress.md` | promote as **SPEC-003**, taking its number at promotion | drafted during this slice as its working authority; §7's sixteen rows each name a real test function and file, checked here, and R-5 correctly reads *review, not a test*. `docs/AGENTS.md`: a slice does not close holding an unpromoted draft. **Promote §6.3's closing sentence as a criterion, not as a row note** (`review-code.md` F-15): *"A clause of this paragraph that says a refusal always reaches a person is making a claim about who decides it… One that cannot say which side decides is a clause that has not been checked."* It is what makes the next instance of that class findable by reading, and F-15 found two instances of it in one sweep | [ ] |
| `docs/adr/005-…md` (**new**) | write: *the event envelope normalizes in stratum 2.* | ADR-001 §Decision names wire-to-canonical normalization in stratum 1 and event ingress in stratum 2, and the envelope is both. `design.md` D-3 records the decision and §10 says it is owed an ADR, because **no ADR-001 instrument sees this choice** — it could be reversed by accident | [ ] |
| `docs/slices/004/design.md` §5.2 | amend `Refusal`'s payload list: five payloads, the fifth `Unavailable(UnavailableCause)` | not canon, but stale about the tree. The list shows four payloads and omits `Unavailable`, which PHASE-03 read as specifying a unit variant; F-a showed that contradicts §5.4's own sentence two paragraphs later, and the tree now carries the payload. See *Design drift* below | [ ] |
| `docs/slices/004/design.md` §10, CD-3 row | *"AC-6's **third** test"* → the second, §9 (ii) | a stale ordinal. §9's own AC-6 row, `canon-delta.md` CD-3, `plan.md` and `slice-004.md` all say (ii), and so does the implemented test | [ ] |

**Design drift not reconciled: four items.** The first is the one a
Reconciliation row above proposes to close rather than to leave; the second was
**added 2026-09-09** (`review-code.md` F-17, whose parent [[F-3]] promised the
entry and whose repair pass did not write it). The third and fourth are
[[F-18]]'s, added the same day, and the fourth is the most consequential of the
four: the other three **outdate a description**, and it **reverses a
decision**.

`design.md` §5.2's interface block lists `Refusal`'s payloads as four —
`TooSoon`, `TooLarge`, `TimedOut`, `InvalidEnvelope` — omitting `Unavailable`
entirely. PHASE-03 read that as specifying a **unit** `Unavailable` and built it
so, recording the reading in its own Assumptions. PHASE-04's finding **F-a**
then showed the reading contradicts §5.4's own sentence — *"`unavailable` means
one thing about the host and three about why … and `detail` says which"* —
which a unit variant's `Display` cannot satisfy. Resolved by orchestrator
ruling: `Refusal::Unavailable` now carries `UnavailableCause`
(`crates/goad-shell/src/ingress/mod.rs:268-295`), and
`unavailable_s_two_causes_carry_different_detail` holds it. **The tree makes
§5.4's sentence true where the design's own payload list, unamended, keeps it
false.** The design is left as written here, because retro-fitting it silently
is exactly what `docs/AGENTS.md` forbids; the amendment is a Reconciliation row
for the user.

**Second — `design.md:229`'s *"when the loop was idle"*.** §5.2's reply table
qualifies the ingress-stopped `unavailable` as reported *"when the loop was
idle"*. `review-code.md` F-3 showed that qualification recorded an
**implementation accident as intent**: the loop folded that refusal in the inner
arm and never presented it, so during an exchange `absorb` was guaranteed to
supersede it before any frame, and the permanent condition R-15 calls
*"the only report there is"* was reported nowhere at all. The repair made it
unconditional — the inner arm presents on its `None` branch — and
`draft-spec.md` §5 and §6.3 now say so in terms, held by
`ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface`.

`design.md` is **left as written**, and deliberately: `git log 93abab3..HEAD --
docs/slices/004/design.md` is empty. `docs/AGENTS.md:168` requires exactly that
— the design is a record of intent at a point in time, and where the
implementation departed and the design stands as written, the departure is said
here rather than edited away. So the design contradicts both the code and the
draft spec on this point, on purpose, and this paragraph is the record of it.
Unlike the first item it has **no Reconciliation row**: nothing is proposed to
the user, because nothing should change.

**Third — the reclaim probe's side effect, which no longer happens**
([[F-18]]). `design.md` §5.5's edge-case table has *"zero bytes, then EOF →
`malformed` — an empty payload is not a JSON document"*, and until this repair
the host's own reclaim probe was one of that row's instances: a second `goad`
start opened a bare `connect` to the live host's socket and sent nothing, so the
live host answered its own would-be replacement `malformed`. `reclaim`'s doc
comment said so in terms and cited the table. **Liveness is now a lock and
nothing connects at all**, so a second start costs the running host nothing —
the row stays true about *writers*, and loses its one host-authored instance.
Two sentences go stale in the same direction and are left as written with it:
§5.5's *socket unlinked underneath a live listener* row offers *"a second host
reclaiming the path under `draft-spec.md` R-3"* as one of its two causes, which
a second host can now reach only if the lock file has been removed first; and
§5.2's `IngressError` sentence lists six faults, where the tree has seven
(`LivenessUnknown` — *whether a live host holds it could not be determined*).
The record of the instance's disappearance is in `slice-004.md` Follow-ups,
under [[F-9]] — which named it as the non-adversarial case an operator meets
first, and now says what became of it.

**Fourth — `design.md` D-10, reversed** ([[F-18]]). D-10 decided *"the
probe/bind race is documented, not closed"*, and named the alternatives it was
declining: *"an atomic `link`; a lock file. Both are partial single-instance
enforcement under another name."* §6's OQ-6 row records the same disposition —
*"answered by not closing it"*. **The repair takes the second alternative**: an
exclusive advisory lock on a sidecar file, held for the host's lifetime, which
is a partial single-instance guarantee inside the ingress module — exactly what
D-10 declined, for exactly the reason it named.

Unlike the three items above it, **this is not a description falling out of
date. It is a decision being taken the other way**, and it is recorded as a
design change rather than absorbed into a repair (`CLAUDE.md`: breaking one of
the five is a design change, not a refactor). The ground is stated rather than
assumed: D-10 declined on **scoping** grounds — a preference about how much this
slice should own — and [[F-18]] rests on **R-3 being broken in shipping code,
measured across three implementations**. A stated invariant that does not hold
outranks a scoping preference, and the repair's goal is a sound liveness
inference; the single-instance guarantee is a **consequence** of holding the
lock for the process's lifetime, not the thing that was wanted. `slice-004.md`
Follow-ups says so under *Single-instance enforcement*, so whoever picks that up
knows a piece of it already exists and where.

`design.md` is again **left as written**, for `docs/AGENTS.md:168`'s reason and
with no Reconciliation row: D-10 records what was decided at design time, which
is true, and a decision that no longer holds is superseded rather than edited.

Three further departures are **amendments, not drift**, and are named here so
they are not rediscovered as drift: `design.md` §5.4's startup order and §9's
AC-10 and AC-9 rows were amended before any phase wrote code, with
`draft-spec.md` §7's R-2 and R-4 rows, because they prescribed mechanisms this
workspace cannot legally implement — no safe umask API, `Host::new` consuming
`Config` before the bind could read it, and no test target linking the binary.
Recorded in `design-log.md`, 2026-09-08. The tree was measured against the
amended text throughout.

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-004.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-004.md` stage set to `done`

Left unticked entire: the code review has not run, so no line of this
checklist is worked yet. The evidence above already answers two of them — the
acceptance criteria and the gate — and the close stage is where they are
ticked, not this one.
