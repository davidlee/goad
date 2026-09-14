# Audit & reconciliation — Slice 005: `goad emit`

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `03b0286..c785f13` on `main` — design and plan at `03b0286`, then
eleven commits: three review-design rounds, four phase-plan commits, four phase
commits, and PHASE-04's AC-8 observation. `git log --oneline 03b0286..c785f13`.

**Question.** For this slice to be finished, all of the following must be true.
Each line names what I intend to *do* about it, written before reading a line of
the implementation.

1. **The gate is green as canon defines it.** POL-001 §Compliance is six
   commands in one order. *Check:* run `just check` to exit 0, and check
   `just -n check` still prints POL-001's block in POL-001's order — a gate that
   has quietly drifted from its policy is a gate that verifies a different thing
   from the one canon claims.
2. **Every AC is met, and its evidence holds what the walk says it holds.**
   `notes.md` F-04-3 is already a walk of AC-1..AC-8 naming a test apiece.
   *Check:* not by reading the walk. For each named test I will open the function
   and read its assertions, asking the question `docs/memory/a-green-test-can-assert-a-proxy.md`
   asks — would this case survive the regression it is cited against? The two I
   expect to be softest are **AC-6** (the case must assert a *normalized `Event`*,
   not emit's own bytes — plan S-4) and **AC-2**'s "nothing branches on `detail`",
   which the Coverage table declares review-held; a review-held clause with no
   argument written down is not discharged, so I will supply the reading or call
   it not met.
3. **Every VT/VA/VH in `plan.md` is discharged where the plan says.** Four
   phases, 22 criteria. *Check:* each to a named test function, a named file, or
   a named argument — and where a phase sheet ticked a box on prose rather than
   on code, I say so.
4. **Nothing was touched that no phase declared.** *Check:* `git diff --stat`
   over the range against the four Surfaces lists, both directions. An undeclared
   path is the strongest lead there is; a declared-but-untouched path is either
   dropped work or a stale plan, and I report which.
5. **The five `CLAUDE.md` invariants survived.** Specifically: (a) no domain
   vocabulary in the new member — and I check the *instrument* covers it, not
   only the sources; (b) permissive wire, canonical inside — `read_reply` must
   not require `protocol`, and the absence of `accepted` must be a *named breach*
   rather than a guess (D-11); (c) a backend failure never takes the host down —
   here read as its client-side dual, that no reply shape and no socket state
   panics emit or leaves the host unable to serve the next connection, so I go
   looking for `unwrap`, `expect`, indexing and `process::exit` in the new crate
   and in `client.rs`; (d) strata run one way — `goad-emit` is stratum 3, names
   only downward, and nothing names it; (e) wire compatibility is never narrowed
   to what the current consumer implements.
6. **Canon is true about what shipped.** *Check:* POL-001 §Verification's four
   instruments plus the vocabulary scan, held against `crates/goad-boundary/tests/checks/*.rs`
   as they now stand with a fifth member in the workspace — the question being
   whether this slice made any sentence of POL-001 or ADR-003 untrue, not whether
   the instruments are as strong as one might wish. Where canon is stale it
   becomes a proposed Reconciliation row, unticked; nothing is written into
   `docs/{specs,policy,adr}/` by this audit.
7. **The residues are stated, not discovered.** Three are on the table already —
   stratum 3's uninstrumented crate edge, ADR-003's "four members", and
   `SendFault::Faulted` having no case at any tier. *Check:* confirm, refute or
   sharpen each against the code, and rule on whether each is acceptable residue
   or a gap. A residue someone wrote down is not thereby discharged.

**Out of scope, deliberately.** The adversarial code review is a separate agent
into `review-code.md`; its findings and its lines of attack are not restated
here, and the Verdict waits on its synthesis.

<!-- This is the audit's scope — evidence, criteria, canon. The code review's
     own lines of attack belong in `review-code.md`'s Brief, not here. -->

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

### The gate

`just check` at `c785f13`, worktree clean but for the concurrent
`review-code.md`: **exit 0**. `just -n check` prints POL-001 §Compliance's six
commands in POL-001's order, unchanged — the recipe has not drifted from the
policy it mirrors.

Counts, per target, from the `cargo test --workspace` leg: `goad` lib 16,
`goad` event-loop 1, event-loop-schedule 1, renderer 156, `goad-boundary`
checks 43, **`goad-emit` unit 31**, **`goad-emit` binary 7**, `goad-semantics`
lib 30, protocol 5, `goad-shell` lib 61, integration 93, shape 6 — **450
passing, 0 failed, 0 ignored**. The stratum-1 leg (`cargo test -p
goad-semantics`) runs 30 + 5 of those again with stratum 1's own feature set.
Transcript: `…/scratchpad/gate-audit.txt`.

### Acceptance criteria

`notes.md` F-04-3 already walks these. This walk **re-derives them from the
code**, because a walk that names a test is worth exactly what that test
asserts (`docs/memory/a-green-test-can-assert-a-proxy.md`). Every named
function was opened and read.

- **AC-1 — met.** `client.rs::send` writes `Event`'s own serialization plus
  `\n` and shuts the write half; `main.rs::envelope` is the §6.2 object.
  `a_sent_envelope_is_accepted_and_reaches_the_judge_as_the_event_it_was`
  (`tests/integration/ingress.rs`) asserts the **normalized `Event` the real
  listener produced equals the one sent**, and
  `an_accepted_envelope_exits_0_and_says_nothing`
  (`crates/goad-emit/tests/binary/exchange.rs`) asserts exit 0 with empty
  stdout from the built binary. The timestamp clause is held transitively: the
  real `envelope::normalize` enforces R-10/R-22 and admits what emit writes
  (AC-6's case). Measured at audit, the bytes of a real invocation:
  `{"source":"reddit-watcher","kind":"reddit-opened","timestamp":"2026-09-14T01:14:32.964200813Z","data":{"count_last_hour":4}}\n`
  — `Z` is RFC 3339's UTC offset designator, so the explicit-offset rule is
  satisfied, not sidestepped.
- **AC-2 — met, including the review-held clause.** Exit 1 and the token:
  `a_refusal_exits_1_with_the_reason_token_on_stderr` and
  `a_too_soon_refusal_also_shows_retry_after_ms` (binary tier);
  `every_refusal_the_host_can_send_is_reported_by_its_token` covers six
  scripted tokens and, with `a_reserved_source_…` and `too_soon_carries_…`,
  the closed eight. The unknown ninth: `an_unknown_reason_token_is_carried_verbatim`
  (`client.rs`) and `a_reason_token_this_client_does_not_know_is_still_a_refusal`
  (integration). **The "nothing branches on `detail`" clause is discharged by
  this audit's reading**, which the Coverage table said review would supply:
  `detail` is read in exactly two places in the workspace —
  `client::read_reply` copies it into `Answered::Refused` unexamined, and
  `render::refused_line` interpolates it into a format string. There is no
  `match`, no comparison and no conditional on its contents anywhere. R-14
  holds.
- **AC-3 — met on the exit code; one message misattributes.** Exit 2 for
  nothing listening, for a usage error and for a §6.3 breach:
  `a_path_with_nothing_listening_exits_2_and_names_the_path`,
  `a_usage_error_exits_2_before_anything_is_opened`,
  `a_reply_that_breaches_6_3_exits_2_rather_than_1`. Every fault line names the
  path and the five read differently from one another
  (`every_send_fault_names_the_path_and_which_fault_it_was`). **The exception**
  is a reply carrying bytes that are not UTF-8: `read_line` returns
  `InvalidData`, `send` maps it to `SendFault::Faulted`, and the caller is told
  *"the connection to … faulted"* when the connection was fine and the host
  wrote something that is not a reply. Measured at audit against the built
  binary — see the `Faulted` note below. Exit 2 is right; AC-3's "naming which
  of those happened" is not met on that one path. The no-deadline clause is
  code, not a test (D-10): nothing in `send` or `main` sets one.
- **AC-4 — met.** One rule, one definition: `goad_shell::config::default_path`,
  five rows tested in `config.rs`, and `startup::arguments`' `[]` arm now calls
  it rather than restating it. The four exits at 2 are
  `no_discoverable_path_is_a_named_fault`,
  `an_absent_configuration_file_is_a_named_fault`,
  `an_unparseable_configuration_is_a_named_fault`,
  `a_configuration_with_no_ingress_section_is_a_named_fault`, each rendered by
  `every_startup_fault_names_the_path_and_what_was_wrong`. `--socket`
  short-circuits: `a_socket_flag_wins_over_a_configuration_naming_another_path`
  and `a_socket_flag_is_honoured_with_no_configuration_at_all`, the second
  passing a closure that answers `None` to everything, so "consults no
  configuration at all" is asserted rather than described.
- **AC-5 — met, and met the right way round.**
  `a_reserved_source_is_sent_and_the_host_s_own_refusal_is_reported` sends a
  real `source: "host"` to a real `bind`ing listener and asserts both sides —
  the client reads `reserved_source`, and the listener's own record shows it
  refused. Nothing client-side pre-empts R-13: `args::parse` and `send` have no
  `"host"` literal anywhere.
- **AC-6 — met, and not a proxy.**
  `the_bytes_on_the_socket_normalize_to_the_event_that_was_sent` passes the
  bytes the *built binary* put on the socket through the real
  `goad_shell::ingress::envelope::normalize` and asserts `source`, `kind` and
  `data` off the resulting `Event`. The subject is the normalized value, never
  the JSON text — S-4 respected. `notes.md` F-04-2 records six mutations to
  `main.rs`, of which swapping `source`/`kind` and dropping `--data` red this
  case alone.
- **AC-7 — met.** `crates/goad-emit/Cargo.toml` names `goad-shell`,
  `goad-semantics`, `serde_json`, `[lints] workspace = true` and a `[[test]]`
  target. No `slint`, no `tokio`, no `jiff`, no `[dev-dependencies]` table at
  all. `cargo tree -p goad-emit` re-run at audit: **zero occurrences of
  `slint`**. The crate edge holds it — but see the instrument table below for
  exactly how far that holding reaches.
- **AC-8 — met.** Observed by the user, 2026-09-14; recorded below at PHASE-04
  because a human observation cannot be reconstructed at audit.

### Verification criteria

Twenty-two across four phases. All discharged; three carry a qualification.

- **PHASE-01** — VT-1 five rows, `config.rs` tests `an_absolute_xdg_config_home_is_the_path`
  … `with_neither_variable_there_is_no_default_path`. VT-2 `wire.rs`'s four
  cases (`an_empty_object_parses_and_carries_nothing` is the "neither
  `protocol` nor `accepted`" row). VT-3 `the_reply_s_bytes_are_exactly_these` —
  and it is *stronger* than the criterion asked: it asserts the bytes, where
  every prior assertion in the workspace read this reply through
  `serde_json::from_str` and could not tell `1` from `null`.
  **VA-1 re-derived at audit from the diff, not taken on trust:** `crates/goad`
  changed at its call sites and their imports, plus the deletion of `clock.rs`
  — and two further files EX-7 names in advance (`Cargo.toml`'s justifying
  comment, `lib.rs`'s `pub mod` line). Nothing else.
- **PHASE-02** — VT-1..VT-4 are the seven new integration cases; VT-5 is the
  eight unit cases in `client.rs::tests`, with `{}` landing on `NonConforming`
  and not on `Unreadable`, as §5.5 said it would. **VA-1 is the one criterion
  whose evidence is testimony rather than artifact**: the injection pass is
  recorded in `notes.md` PHASE-02 and cannot be re-derived from the tree. The
  record is specific enough to be checkable in principle (it names which
  mutation reds which case, and the `timed_out` message it produced), and it
  produced a *correction* — the R-6 finding — which is what a real injection
  pass looks like and a fabricated one does not.
- **PHASE-03** — VT-1 thirteen `args` cases including the non-UTF-8 row; VT-2
  `every_usage_error_names_the_flag_and_reprints_nothing`, **all eight variants
  enumerated in one array**; VT-3 the two `--socket` cases; VT-4 four
  `refused_line` cases; VT-5 `the_envelope_carries_exactly_the_four_keys`,
  asserting the sorted key set by equality rather than by containment; VT-6
  `every_startup_fault_names_the_path_and_what_was_wrong`, **all five**
  variants; VA-1 the manifest, read at audit.
- **PHASE-04** — VT-1..VT-4 the seven binary cases; VA-1 `cargo tree`, re-run;
  VA-2 is this walk; VH-1 the user's own observation.

### Surface delta

`git diff --stat 03b0286..c785f13` over non-`docs` paths: 29 files, +2174/−76.
Held against the four Surfaces lists:

**Undeclared, and each accounted for.** Three paths appear that no phase's
Surfaces list names, and none is scope creep:

- `Cargo.lock` — the mechanical consequence of PHASE-03's one
  `workspace.members` entry; three lines naming `goad-emit`'s three
  dependencies and nothing else.
- `crates/goad/Cargo.toml` — one comment. PHASE-01/EX-7 names this file
  explicitly as a prose site whose sentence ("`clock.rs` is why `crates/goad`
  carries `jiff`") the lift falsifies; tidying it is S-2's stated allowance.
  The entry stays justified and the comment now says why
  (`diagnostics.rs`'s `TimestampRound`). **Declared in the criterion, missing
  from the Surfaces line** — the surfaces list is what is wrong here, not the
  edit.
- `crates/goad/src/lib.rs` — this *is* in PHASE-01's Surfaces list. Noted only
  because it is the one line that deletes a module declaration.

**Declared but untouched: none.** Every path in every Surfaces list was
edited, including the five test files PHASE-01 bounded to `use` lines — whose
diff is, as EX-6 required, five import lines and two doc-comment lines with no
assertion and no fixture altered.

### Instrument reach over the new member

The PHASE-03 finding on this is **half right and mis-names its subject**, so
the audit states it exactly rather than repeating it.
`crates/goad-boundary/tests/checks/structure.rs` is **not** "the crate-edge
instrument" and is not one of POL-001's four: it is slice 002/003's structural
scan, and its subjects — `quit_event_loop` having one call site, no
`tokio::spawn` handle, `slint::spawn_local` being the one spawn, the
identifier `resolve` — are facts about a renderer. They have no referent in
`crates/goad-emit`, which has no event loop and no scheduler. Its not reaching
the new member is not a gap in it.

What actually reaches `crates/goad-emit`, read off
`crates/goad-boundary/tests/checks/*.rs`:

| instrument (POL-001 §Verification) | reaches `crates/goad-emit`? |
|---|---|
| ADR-001's direction rule at **crate edges**, by Cargo resolution | **yes.** Its manifest names three crates, none of which is `slint` or `tokio`, so naming either in its sources is `error[E0433]`. This is AC-7's whole argument and it holds. |
| the **manifest allowlist** (`allowlist.rs`) | **no**, by POL-001's own scoping — it holds an entry "in a stratum 1 or 2 manifest", and names `goad-semantics`'s and `goad-shell`'s manifests by path. `goad-emit` joins `goad` as an unbilled stratum-3 manifest. |
| the **stratum 1 purity scan** (`purity.rs`) | **no**, and could not: its subject is `crates/goad-semantics/src` by definition. |
| `cargo test -p goad-semantics` | n/a — it builds stratum 1 alone and rejects nothing. |
| the **domain-vocabulary scan** (`vocabulary.rs`) | **yes, automatically.** It reads `workspace.members` for itself (`:45-58`) and also checks each member's own crate name in its manifest. The new member arrived covered with no edit. |

**POL-001 needs no amendment.** Every sentence in its Verification section is
as true of a five-member workspace as of a four-member one; the slice made
none of them untrue. That is the question this audit was to settle, and it is
settled in the negative.

**The residue, stated precisely.** The crate-edge instrument's coverage of
`goad-emit` is *derived from its manifest*, and the manifest is the one thing
no instrument checks at stratum 3. One line in `crates/goad-emit/Cargo.toml`
dissolves AC-7's guarantee, and nothing but review would notice — exactly as
has been true of `crates/goad` since 002. OQ-3 said this in advance and
`slice-005.md` Follow-ups carries it. The slice did not create the gap and did
not widen it; it added the second instance, which is the better argument for
taking the Follow-up.

### `SendFault::Faulted` — the residue is narrower and sharper than recorded

`notes.md` says the variant "is constructed in production and exercised by no
case", with "no deterministic trigger at this tier". Both halves need
correcting.

- It **is** exercised at the render tier:
  `every_send_fault_names_the_path_and_which_fault_it_was` constructs it and
  asserts its line is distinct from the other four. What has no case is
  anything that makes `send` *produce* it.
- A **deterministic trigger does exist**, and it is one line of fake listener.
  `send` reads with `BufRead::read_line`, which requires UTF-8 and returns
  `ErrorKind::InvalidData` when it does not get it; `send` maps that to
  `Faulted`. Measured at audit against the built binary, with a fake listener
  replying `b"\xff\xfe\n"`:

  ```
  goad-emit: the connection to …/f.sock faulted: stream did not contain valid UTF-8
  EXIT=2
  ```

  No race: the peer writes, then closes, and the client is already blocked in
  the read.

So this is a **gap with a named cheap case**, not unreachable residue. It also
surfaces the AC-3 mismatch above: the variant's message attributes to the
transport a fault that belongs to the host's bytes, which is the one thing
`SendFault`'s own doc says each variant must get right ("each names which side
was at fault"). The remedy is the review's to disposition — a case at the
`goad-shell` integration tier, plus either a reworded `Faulted` or a sixth
variant — and it is **not** a blocker on the slice's stated scope: `plan.md`
VT-4 asked for `Unreachable` and `NoReply` and got both.

### Other findings

- **`wire::Reply`'s doc contradicts its own attributes.**
  `crates/goad-shell/src/ingress/wire.rs:25` says *"No field carries
  `#[serde(default)]`"*; three fields below it carry
  `#[serde(default, skip_serializing_if = "Option::is_none")]`. The **code is
  right** and follows PHASE-01/EX-4 as written; the doc comment states a
  decision that was not taken, and `notes.md`'s PHASE-01 "Decisions" records
  the same untaken decision. Behaviour is identical either way — the attribute
  is genuinely inert on an `Option`, which is what the comment's reasoning
  says — so this costs nothing at runtime and everything in trust: it is a
  false statement on a wire type, in the sentence a future reader would rely
  on when deciding whether a field is absent-tolerant.
- **A rationale detached from its assertion.**
  `crates/goad-shell/src/ingress/mod.rs:925-928` — §6.3's newline argument
  (*"this is the function that owes the byte"*) now heads
  `the_reply_s_bytes_are_exactly_these`, and
  `a_reply_is_one_newline_terminated_line`, which it was written for and which
  still sits at the foot of the module, has no doc comment at all. The new
  case was inserted between the comment and its test.
- **`allowlist.rs`'s module doc enumerates two exempt members.**
  `crates/goad-boundary/tests/checks/allowlist.rs:8-11` reads *"`goad` and
  `goad-boundary` carry no allowlist here"*. There are now three, and
  `goad-emit` is not named — the same shape of staleness as ADR-003's, one
  stratum down, and the place a reader goes to learn what the instrument
  covers.
- **Two doc-table rows with no case.** `args::parse`'s table promises
  *"`--help` wins if both appear"* and `help_is_help_wherever_it_appears`
  never passes `--version`; `default_path`'s table says `XDG_CONFIG_HOME`
  *"unset, empty or relative"* and the empty row has no case of its own. Both
  are one-line additions. PHASE-03/VT-1 asked for "one case per doc-table row",
  which is met at row granularity and not at clause granularity.
- **The binary tier has no `--help` / `--version` case**, which PHASE-03/EX-6
  states as behaviour and no VT asked for. Verified by hand at audit instead:
  `--help` prints the usage block to stdout and exits 0; `--version` prints
  `0.1.0` and exits 0; `--help` beside a well-formed send still yields help.
- **No bound on the reply emit will read.** `read_line` grows a `String`
  without limit, so a host that streams forever makes emit allocate forever.
  Deliberate in spirit — §6.4 makes the *wait* unbounded by contract, and R-7's
  byte bounds are the host's and not emit's to second-guess (design §5.5) — but
  the unboundedness of the *buffer* is a different thing from the
  unboundedness of the *wait*, and no document says so. Noted as an
  observation, not a finding: the peer is a local host the caller chose.

### AC-8 — a person ran it

<!-- Recorded at PHASE-04 (VH-1), not at audit: a human observation cannot be
     reconstructed afterwards. `docs/AGENTS.md` §Tiers — a slice does not close
     until a person has run the software and seen the new behaviour. -->

**2026-09-14.** `just demo` in one terminal, then `--source hand --kind poke`
against `./goad-demo.sock` from another — `just emit hand poke`, which is
PHASE-04's recipe for VH-1's own
`cargo run -p goad-emit -- --socket ./goad-demo.sock …`. The user's account: *"an event arrived: hand/poke — buttons
replaced w/ ok button"*.

That is `examples/shell/backend.sh`'s **event** branch, and it is what makes
this the backend leg rather than a repaint. Two things distinguish it from the
prompt the demo shows on its own:

- the title interpolates the `source` and `kind` emit sent — `hand / poke` —
  so the envelope reached the backend with its fields intact;
- the option set changed from the host-originated prompt's two (`Yeah` / `Nah`)
  to the event view's one (`OK`), so the view the user saw was composed from
  *this* evaluation's response.

The leg no test target covers — emit → socket → `normalize` → backend
invocation → the view — is therefore observed end to end. AC-6 holds the
upstream half of it under test (`slice-005.md` AC-6 is deliberately not phrased
as "reaches the backend"); this is the rest.

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** **resolved** · outstanding blockers: **none**, and none at any
  point across four rounds. Eighteen findings: three major, nine minor, six
  nits. Sixteen `verified` by a review round; F-17 and F-18 repaired and closed
  by the user's disposition without a fifth round — recorded in the ledger's
  Closing note, which is the one concession this closure makes knowingly.
  None contested, none withdrawn, and no repair needed repairing.

## Verdict

**The slice does what it set out to do.** `goad-emit --source S --kind K` puts
one SPEC-003 §6.2 envelope on the host's socket and returns an exit code that
says *who was wrong*: 0 the host took it, 1 the host refused it and said why, 2
no usable answer could be had. A person ran it and the view changed (Evidence,
AC-8). The cron line the Purpose section wanted exists, and nothing in it knows
the wire format.

**What the audit and review actually bought.** The slice was green at `c785f13`
by its own gate, its own acceptance criteria and a human demonstration — and it
still carried three defects that would have shipped:

- a **wire narrowing** that refused conforming hosts. `Option<u8>` on a field
  the client's own doc said it did not read, so `{"protocol":1.0}` — the same
  number, JSON having one — came back as *"is not one JSON object"* about a
  document that was one. This is the failure `CLAUDE.md` names as the reason
  the project exists, found in the slice that implements the contract's client
  half;
- a command line that **exited 0 having sent nothing**
  (`--data --version`), spending the one code reserved for "the host accepted
  it";
- an **unbounded read** of the reply, where SPEC-003 bounds the host's own read
  at 64 KiB and says why.

None was reachable from the acceptance criteria as written. AC-1 through AC-8
are all met, and were met before the review as well as after — which is the
audit's most useful negative result: **a criterion set can be complete and
honest and still not reach the contract's encodings and bounds.** The defects
clustered there, not in the semantics.

**What the audit confirmed, and it is the part most at risk.** PHASE-01's four
lifts are genuine lifts — `clock.rs` byte-identical to its predecessor modulo
doc comments, `default_path` condition-for-condition, and `wire::Reply`'s one
real change pinned by an exact-byte characterization case written *before* the
lift. A lift that changed behaviour while claiming to be a move would have been
the worst thing in this diff and there is none. The strata hold: `cargo tree -p
goad-emit` has zero `slint`, nothing names the new member, and the
domain-vocabulary scan reached it on arrival by reading `workspace.members`.
AC-5 and AC-6 are held honestly rather than by proxy.

**Accepted knowingly.**

1. **Three declared-field shapes still report a coarse sentence** — nesting past
   128 levels, a number outside `serde_json`'s range, a duplicated declared
   name, and malformed string content. Each is exit 2 with the host at fault
   either way, so what they cost is the precision of one sentence and never a
   caller's decision. The boundary that *is* held is stated generatively rather
   than as a list, because the list was completed three times and a shape
   outside the hypothesis set turned up each time.
2. **`arbitrary_precision` is declined**, not open. Closing the range shape
   needs that feature workspace-wide, changing `Number`'s representation and
   `PartialEq` for `goad-semantics` too — plan STOP condition S-5, raised by
   the repair agent rather than taken, and declined by the user as a bad trade.
3. **AC-2's last clause is review-held by design.** *"Nothing branches on
   `detail`"* is an absence of code; `detail` is read in exactly two places
   workspace-wide and neither matches on it. SPEC-003 §7 handles such clauses
   the same way.
4. **`SendFault::Faulted` is constructed in production and reached by no case.**
   Its one deterministic trigger was removed by F-5's repair, which was the
   right repair — the fault was an attribution error, not a missing case.
5. **An unreachable arm is kept deliberately** in `one_json_object`, as a bound
   on a future defect rather than a path, and nothing can test it.
6. **F-17 and F-18 were repaired without a review round**, per the user's
   disposition. Both are sentences; neither can introduce a behavioural defect.
7. **AC-8 remains the only end-to-end evidence.** No test target links the CLI
   binary and a running host, deliberately — `slice-005.md` AC-6 is phrased to
   say so rather than to claim the backend leg it cannot reach.

**A stratum-3 manifest is checked by nothing but review.** Not a defect of this
slice — `crates/goad` has been in that position since 002 and `goad-emit` is
the second instance, which is the better argument for the standing Follow-up
than either alone. ADR-003 said the opposite and has been corrected.

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

All four rows are **proposed and unapplied**. Nothing in `docs/specs/`,
`docs/policy/` or `docs/adr/` was written by this audit; every row below needs
explicit user endorsement first. This slice holds no `draft-spec.md` and no
`canon-delta.md`, so there is nothing to promote or abandon.

| document | change | reason | done |
|----------|--------|--------|------|
| `adr/003-…md` §Decision | "a Cargo workspace of **four** members" → five, with `crates/goad-emit` added to the enumeration as stratum 3, the `goad-emit` binary; the `crates/goad` line reads "the Slint renderer and the host binary" | Stale prose in a record of a decision taken at 002. **Not a reversed decision**: ADR-003's rule is one crate per stratum with stratum 3 holding entry points, and 005 added a member by following that rule. No superseding ADR is called for. The count must not be confused with POL-001's *four ADR-001 instruments*, which is a different four and is unchanged | [ ] |
| `adr/003-…md` §Alternatives considered | the "A fifth crate, one per stratum-3 concern" entry, whose stated ground is "T2 (a second binary) has not fired" | T2 fired in 005. The alternative itself stays **rejected** — what it proposed was splitting the *renderer* from the *host binary*, which 005 did not do — but its reason is now historically false, and an ADR whose rejection rests on a false premise is the exact trap ADR-003 §Context says ADR-002 laid. The record should say T2 fired and was answered by a new member for a new binary, not by splitting `crates/goad` | [ ] |
| `adr/003-…md` §Consequences/Negative | the sentence "a new workspace member needs its own entry in the manifest allowlist and its own reach in the vocabulary scan's walk, and nothing but review catches a member added without either" | Both halves are untrue of the code. The vocabulary scan reads `workspace.members` for itself (`vocabulary.rs:45-58`), so a new member arrives covered; and no stratum-3 member carries an allowlist row, by POL-001 §Verification's own scoping to "a stratum 1 or 2 manifest". `crates/goad-emit` is the **second** member for which the sentence is false. OQ-3 and D-8 argued this in the open and the user ruled against amending mid-slice; what is left is the wording, and audit is where it belongs. The replacement states the real residue: a stratum-3 manifest is billed by nothing but review, which is `slice-005.md`'s standing Follow-up | [ ] |
| `policy/001-the-phase-gate.md` | **no change** | Recorded as a row deliberately, so that no one later amends it by analogy with ADR-003's. Every sentence of §Verification — the four ADR-001 instruments, the vocabulary scan, the residue, and what each does *not* reach — is exactly as true of a five-member workspace as of a four-member one. The slice added no instrument, changed none, and made none of the policy's claims untrue | [x] |

Four further staleness items are **code, not canon**, and belong to
`review-code.md`'s disposition rather than to this table: `wire.rs`'s doc
comment, `allowlist.rs`'s two-member enumeration, the detached rationale in
`ingress/mod.rs`, and `notes.md`'s PHASE-01 Decisions entry recording a
decision the code did not take. They are stated under Evidence with their
locations.

**Design drift not reconciled.** `design.md` is left as written in all six
places. Each is a spelling or a locality the design did not reach, none
changes what the design decided, and `notes.md` records five of the six as
decisions taken in the open during execution:

- **`StartupFault` has five variants.** `design.md` §5.2's inline comment names
  three and `plan.md` PHASE-03/EX-5 names four; the fifth is `ClockUnreadable`.
  §5.4's own sequence reads a clock between the arguments and the socket and
  names no renderer for its failure, so the variant is the design's diagram
  catching up with the design's prose. AC-4's four still each render a path;
  the clock renders neither path nor flag, because neither is what went wrong.
- **Everything in `goad-emit` is `pub(crate)`**, where §5.2 writes `pub fn
  parse` and friends. `clippy::unreachable_pub` refuses `pub` in a binary
  crate, and is right to: a binary has no reachable public API. The module
  boundaries are the design's; only the visibility keyword differs.
- **`Answered` derives `PartialEq`, `SendFault` does not.** §5.2 shows no
  derives on either. `PartialEq` is what lets a case assert a refusal whole —
  token, advice and detail in one assertion; `SendFault` carries two error
  types that have none.
- **`client.rs` has a private `answer(&str)` step** between the bytes and
  `read_reply`, which §5.2 does not show. It is what makes
  `SendFault::Unreadable` — a property of bytes that `read_reply(Reply)` never
  sees — reachable without a socket, which is what VT-5 asks for.
- **`client.rs` has a private `envelope_line(&Event) -> String`**, likewise
  unshown. It exists to localise the infallible-serialization claim to a
  function that returns no `Result`, because `clippy::unwrap_in_result` is
  live and unscoped. The alternative was a sixth `SendFault` variant, which
  EX-2 forbids.
- **`render` exports a fifth item, the `USAGE` const**, beside §5.2's four
  functions — the same shape `crates/goad`'s `diagnostics` already has.

## Closure

- [x] All findings dispositioned; no blockers outstanding — eighteen findings
      across four rounds, every one dispositioned by the user before it was
      acted on. No blocker at any point. F-17 and F-18 closed by disposition
      rather than a fifth round, recorded in the ledger's Closing note.
- [x] All acceptance criteria met, or explicitly waived by the user — AC-1
      through AC-8. AC-3's qualification at the time of the audit (a non-UTF-8
      reply blamed on the transport) was **repaired**, not waived: `Faulted` is
      now the transport's alone.
- [x] Tests and checks green — `just check` exit 0; **503 cases**, none
      ignored; 450 at the start of the audit. The recipe still mirrors
      POL-001's six commands in order.
- [x] Specs / policy / ADRs reconciled, with user endorsement where amended —
      three ADR-003 rows applied under explicit endorsement; POL-001 unchanged,
      and the row recording *why* it needs no change is deliberate so nobody
      later amends it by analogy.
- [x] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason
      written down — the slice holds neither. Tier 1, canon impact none, no
      phase hit S-1.
- [x] `slice-nnn.md` Summary and Follow-ups written.
- [x] `notes.md` Harvest current; durable facts lifted to `docs/memory/` — the
      two Harvest entries the audit refuted are struck and corrected in place.
      One new memory (`a-rust-type-on-a-wire-struct-narrows-the-wire.md`) and
      two appended to memories that already owned the ground rather than
      duplicating them.
- [x] `slice-nnn.md` stage set to `done`.
