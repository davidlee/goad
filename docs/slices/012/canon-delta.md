# Canon delta — Slice 012

This file has one entry per affected document (`docs/AGENTS.md` §Canon that
does not exist yet, or must change). Each entry names the document and the
section, gives the change **as it will be stated**, and says why. Nothing here
is applied mid-slice. It is applied at audit, with the user's explicit
endorsement, and recorded in `audit.md`'s Reconciliation table.

**The count rule** (`CLAUDE.md`, name never count). Every count in a passage
this delta touches is either replaced or justified as exempt. Test names are
placeholders from `design.md` §9. At audit each is re-pointed to the symbol
that shipped. Nothing here cites a line number.

`design.md` §6's questions are settled (`design-log.md`, 2026-09-29), and so
are design review's (`design-log.md`, 2026-09-30); the entries below
state their outcomes.

---

## SPEC-001 (the host/backend interaction protocol)

No requirement id is renumbered. One is added: **R-59**, the next free id.

### Change 1 — §4 *Failure*: new R-59

**Why.** SPEC-001 §1 promises that a backend author will "know from the error
they get back which side was wrong". No requirement states it, and nothing in
code carried it (`research.md` R-a). The user decided that the claim is canon,
that SPEC-001 owns it because it owns the taxonomy, and that there are four
sides (`design-log.md` 2026-09-26, OQ-2 and OQ-8). The format of whatever
reports the claim is not canon. Design review rounds 1 and 2 settled the
imprecision one side per kind costs and moved `CleanupFailure::TimedOut`
(`design-log.md` 2026-09-30, *design review round 1: dispositions*, U1, and
*design review round 2: dispositions*). Each round patched a sentence and
left another false of some row, so round 3 reframed the requirement rather
than patching it again (`design-log.md` 2026-09-30, *R-59 reframed;
PipeMissing; F-22; round 2's unbriefed repairs*):

- **Scope by channel.** R-59 covers what the host reports on each channel of
  an exchange or of an answer — a failure, a discarded instruction, a cleanup
  failure, a refused answer — named once as a **refusal**, and not a list of
  other requirements' items, which missed kinds each required by their own
  requirement. What the host reports when it rejects the configuration file at
  load, or a forwarded envelope, is outside it in terms.
- **The id, one clause per case.** Most of SPEC-001's transport and failure
  rows are host obligations a backend cannot break, so the id is never read
  as a rule someone broke. It is the requirement stating the rule the kind
  enforces; R-44 where the kind cannot tell which more specific rule an
  instance broke, or where R-44's list is the only requirement that names the
  refusal (no rule says a command must be spawnable); R-45 for a failure of
  an exchange no requirement names, since R-45 governs what the host does
  with it. R-45 is reworded to cover such a failure whichever side caused it
  (Change 7).
- **Sides by where the cause lies**, one side per kind, the imprecision
  declared.

**As it will be stated** (a new row after R-47):

> | R-59 | Each report the host makes on the channels of an exchange or of an answer — a failure, a discarded instruction, a cleanup failure, a refused answer (below, a **refusal**) — MUST name the **side at fault** and a **requirement** of this spec. The requirement is, by the refusal's kind: the requirement stating the rule the kind enforces; R-44, where the kind cannot tell which more specific rule an instance broke, or where R-44's list is the only requirement that names the refusal; and, for a failure of an exchange that no requirement names, R-45, the requirement governing what the host does with it. The side is where the refusal's cause lies: **backend**, in what the backend sent or did; **configuration**, in the user's configuration, which named something the host could not use; **host**, on the host's side of the seam — its own code, or whoever answered through it; **environment**, in the operating system, which failed the host. The requirement and the side are properties of the refusal's kind and not of the instance, so one kind names one side and one requirement — save where one kind is required by two requirements to be the same error: R-53 requires `fields` on an alternative to be refused as R-50's error is, and that kind's requirement is read off what the error names. Where a kind's cause can lie on another side — a timeout the configuration set too short, a signal sent from outside the backend, a spawn refused for want of resources, a disposal a loaded machine did not finish in time — the kind still names its one side, and the refusal carries what lets a reader see the other: the configured timeout, that the backend was signalled, the operating system's error, the limit disposal was given. What the host reports when it rejects the user's configuration file at load, or a forwarded envelope (SPEC-003), is on no channel of an exchange and outside this requirement. What a refusal names is fixed here; how it is worded is not, and whether a surface that reports it to a person carries both is that surface's own spec's to say. | §7 |


### Change 2 — §7: R-59's row

**Why.** The claim needs an instrument. The compiler holds that it is total;
the corpus witnesses that it is correct.

**As it will be stated** (a new row):

> | R-59 | unit, and a witness. `requirement()` and `fault()` are total matches beside each taxonomy, so a new variant does not compile until both are decided: `crates/goad-semantics/src/error.rs::tests::every_protocol_error_names_a_requirement_and_a_side` and its bounds and schedule siblings, and `crates/goad-shell/src/error.rs::tests::every_backend_error_names_a_requirement_and_a_side` and its cleanup and state siblings. That each **answer is right** is witnessed by the corpus, which states requirements independently of the code and catches an answer outside the fixture's own list: `normalize.rs::every_refusal_fixture_names_a_requirement_in_its_own_list` over the `protocol` and `protocol-text` corpora, and `::every_discard_fixture_names_a_requirement_in_its_own_list` over the discards and the `schedule` corpus. What it does not catch is an answer moved to another id the same fixture also lists — every `schedule` error fixture lists R-25, for one — and review of the tables holds that. The transport, cleanup and state kinds have no fixture, and their answers are held by review of the tables. |

### Change 3 — §7: R-56's row

**Why.** Two facts in the row change. The one place the host names a kind
moves to stratum 1 (`design-log.md` 2026-09-26, OQ-4). The backend tolerance
clause, held today by "review, not a test", becomes testable: `goad-check`
sends an unrecognised host kind, and charges a failure on it to this
requirement only where that failure is evidence of intolerance — the
backend's own, from a backend that answered a known kind without one
(`design.md` §5.2.2; review F-3).

**As it will be stated.** Replace the sentence beginning "unit, at the one
place the host names a kind:" through "…and nothing else." with:

> unit, at the one place the host names a kind: `crates/goad-semantics/src/protocol/canonical.rs::tests::a_scheduled_stimulus_names_itself_scheduled` and `::a_scheduled_stimulus_s_event_carries_the_three_normative_fields`, the second asserting `source`, `kind`, `timestamp` and payload together, against `Stimulus::kind`, which returns exactly `"startup"`, `"requested"` and `"scheduled"` and nothing else.

Replace the sentence beginning "The **tolerance** clause is an obligation on
backends" with:

> The **tolerance** clause is an obligation on backends, which no host test can observe of a backend it does not ship; it is tested of any backend a person points the checker at, since `goad-check` originates an `evaluate` whose kind is none of the three and reports against this requirement a failure on it whose side is the backend's, from a backend that answered at least one of the three without failure: `crates/goad-check/tests/binary/…::a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`, with `::a_backend_failing_identically_on_every_kind_is_not_charged_with_r56` holding the second condition and `::the_probe_kind_is_none_of_the_host_s_own` holding that the probe is an unrecognised kind and not one of the three. Each backend the kit ships is held to it in the gate by `crates/goad-check/tests/kit/…::each_shipped_example_is_accepted_by_the_checker`.

In the emission-half sentence, replace "`Stimulus::kind` is the one place the
host names its own event, `crates/goad/src/wire.rs` the one place it is
written" with:

> `Stimulus::kind` (`crates/goad-semantics/src/protocol/canonical.rs`) is the one place the host names its own event, and `Stimulus::event` the one place it is built

### Change 4 — §7: R-57's row

**Why.** The single site moves to stratum 1 so that the checker's own respond
obeys R-57 without restating it (`design-log.md` 2026-09-26, OQ-4). The row's
argument, "one site is the requirement's own structure", now holds across the
workspace rather than across one crate. The row is rewritten whole: its
opening and closing move with the site, its `datetime` literals move with the
tests (`design.md` §5.2.4), and the counts in its middle ("All five clauses",
"six keys") are of R-16's kinds, an open list (review F-13, F-14).

**As it will be stated.** The row becomes:

> | R-57 | unit, at the **single site** a submitted value is written: `Submitted::to_json` (`crates/goad-semantics/src/protocol/canonical.rs`), a total match over `Submitted`, whose variants are `FieldKind`'s one for one, tested by `canonical.rs::tests::every_submitted_kind_writes_the_json_type_r57_names`. One site is the requirement's own structure — a mapping stated in two places is a mapping that can drift. Both writers go through it: the renderer's `draft::submitted` (`crates/goad/src/draft.rs`) projects its widget state onto `Submitted` and decides no type — `draft.rs::tests::the_projection_to_submitted_is_the_identity_on_each_kind` — and the checker builds `Submitted` directly. What a field nobody touched submits is `Submitted::as_drawn`, which both writers share. Every clause is asserted, from both directions, by a renderer that draws every kind: `crates/goad/tests/renderer/fields.rs::every_untouched_kind_leaves_the_host_with_the_json_type_r57_names` reads the submitted values off the child process's own request log for a form nobody touched, keys each a different JSON type from its neighbour, and `::every_operated_kind_leaves_the_host_with_the_json_type_r57_names` does the same after every control has been operated. The per-kind cases stand beside them: `::an_untouched_datetime_reads_not_set_on_screen_and_submits_the_epoch`, `::choosing_an_alternative_submits_its_id_where_the_field_id_is_the_options_own`, `::an_unbounded_number_submits_what_was_typed_and_invents_no_range`. The `datetime` expected value is partly self-agreeing — it is computed by the production `instant::compose` — and the *format* is pinned by literals at `canonical.rs`, which is where a drift in the RFC 3339 spelling would be caught. The site that must change when the *protocol* grows a new kind is `Submitted::as_drawn`'s match over `FieldKind`, which does not compile until the kind has a `Submitted` variant and an untouched value — and, for the renderer, `view_model.rs::drawn_form`'s, which does not compile until the kind is sorted into drawn or `Undrawn::FieldForm` |


### Change 5 — §7: the paragraphs about rows held by review

**Why.** The intro says "Five rows are held by **review**", and the closing
paragraph says "Five rows … R-56 is a sixth only in part". Both are counts
that nothing re-reads, and Change 3 makes the second one false: R-56's
backend clause is now tested.

**As it will be stated.** The intro sentence becomes:

> Some rows are held by **review** rather than by a test, and each says so and why.

The closing paragraph becomes:

> Nothing here is marked unverified. The rows for R-9/R-19, R-18, R-20, R-30 and R-49 are held by review rather than by a test, each for a reason stated in the row: the subject is a property of the source text or of the contract, not a behaviour anything can execute. R-56's emission half is held by review for the reason its row gives, and so are R-59's answers for the transport, cleanup and state kinds, which have no fixture. R-49 constrains the other side of the seam, and no checker can observe a side effect a backend did not report.

### Change 6 — §4 and §7: R-16 states that a `choice` field offers at least one alternative

**Why.** No requirement stated that a `choice` field's alternatives are
non-empty, though the host refuses an empty list (`EmptyAlternatives`). R-59
needs a requirement stating the rule the kind enforces, and neither R-52
(uniqueness) nor R-44 (whose shape items do not include an empty array) is
one (review F-38; `design-log.md` 2026-09-30, *design review round 2:
dispositions*, reversing round 1's U8). A wording fix: the wire does not move.

**As it will be stated.** In R-16, "a `choice` field MUST carry its own
`options`, whose shape R-53 constrains" becomes:

> a `choice` field MUST carry its own `options`, at least one, whose shape R-53 constrains

In the §7 row for R-13, R-14 and R-16, after the `R-16-a-number-field-with{,out}-bounds` fixtures, add:

> ; R-16's *at least one* by `R-52-a-choice-field-with-no-alternatives`, whose list names R-16, and unit `canonical.rs::an_empty_alternatives_is_rejected_as_alternatives_never_as_options`

### Change 7 — §4 and §7: R-45 covers a failure of an exchange whichever side caused it

**Why.** R-59 names R-45 for a failure of an exchange no requirement names:
`BackendError::Io`, side environment, and `BackendError::PipeMissing`, side
host. R-45's subject is "backend failure", which neither is by its letter, so
R-59 would name a requirement that does not govern the failure
(`design-log.md` 2026-09-30, *R-59 reframed; PipeMissing; F-22; round 2's
unbriefed repairs*; review F-39). A wording fix: the host already behaves so,
since `Host` returns every transport error through `Host::no_action`
without reading the variant. P-C is unchanged: it states the principle for what a
backend sends, and the reworded R-45 is wider than it and does not
contradict it.

The §7 row is restated whole rather than extended (`design-log.md`
2026-09-30, *design review round 3: dispositions*). It counted its witness's
exchanges, and one count was already false: "nineteen … then a successful
one" is twenty, where the test runs a view, one exchange per mode, and the
answer (review F-42). It now names them instead: `PROTOCOL_MODES` and
`TRANSPORT_MODES` are the test's own constants. And the reworded R-45 now
covers `Spawn`, which the witness does not run: the row names the failures
the witness runs and the three it does not, instead of claiming it reaches
every failure a test can provoke (review F-41).

**As it will be stated.** R-45 becomes:

> No failure of an exchange with the backend, whichever side caused it, may terminate the host, and none may leave it unable to invoke the backend again.

The §7 row for R-45 becomes, whole:

> | R-45 | integration: `failure_matrix.rs::one_host_survives_every_misbehaving_backend_and_still_works` — one `Host` and a single parameterized backend: an exchange that presents a view, then one exchange for every mode in `PROTOCOL_MODES` and `TRANSPORT_MODES`, then the answer to that view. Reuse is witnessed by state the failures did not touch, not by the last exchange working: a suite that asserts only the last exchange passes against a `Host` rebuilt every iteration. The failures it runs are the protocol refusals among `PROTOCOL_MODES` and the transport failures `TRANSPORT_MODES` names: a timeout (`Timeout`), an output flood (`OutputTooLarge`), a valid answer disclaimed by a non-zero exit (`ExitStatus`) and a body that will not parse (`Protocol`). It does not run `Spawn`, `Io` or `PipeMissing`. A spawn failure cannot be a mode of one parameterized backend, which must be spawned to be instructed, and no test provokes the other two. Each of the three reaches the caller through the same `Host::no_action` path as the failures it runs — `Spawn` is seen reaching it by `failure_matrix.rs::a_command_that_cannot_be_spawned_reaches_the_caller_as_a_spawn_failure` — and that they leave the host able to invoke the backend again is held by review |

It replaces "one `Host`, nineteen consecutive exchanges against a single
parameterized backend (thirteen protocol bodies and four transport failures),
then a successful one" with the named sequence, and appends the last three
sentences. The reuse sentence is kept as it stands.

---

## SPEC-004 (process exit status)

**Why.** The checker is a new binary, and SPEC-004 owns every binary's status
(AC-4). The user decided to govern `goad-emit` in the same change
(`design-log.md` 2026-09-26, OQ-5), which closes this spec's OQ-2. §3's
principles are unchanged; they already bind every binary. Every passage that
says `goad-emit` is owned and not governed becomes false, and each is changed
below. `design.md` §5.2.5 sets out the checker's classes.

### Change 1 — §Owns and §2 Scope

In §2 *Boundaries*, replace the first two bullets ("`goad-emit` is nominally owned and not yet
governed…" and "`goad-emit`'s classes are not the host's classes…") with:

> - **Each binary, its own cut.** §4 governs each binary this project ships — the host, `goad`; `goad-emit`; and `goad-check` — under its own heading. They share §3's principles and not one another's classes: the host's cut is phase against its event loop; `goad-emit` reports a refusal **as an answer** — the host it wrote to considered the envelope and said no — and a host that never answered has none to give; `goad-check` reports **a verdict** — what the host reported running this command — and a checker that delivered none has none to give. Each cut is still a phase in §5's sense: how far the process got. A new binary is an append to §4 and a table in §6.

Keep the SPEC-001 §2 bullet, which explains why the `goad emit` command line
is this document's to own; its closing clause ("that binary's statuses are
nominally owned here, and not governed") becomes:

> that binary's statuses are owned and governed here.

*In scope*'s "for the host, what each status means and what may and may not
be inferred from it" becomes:

> for each binary §4 governs, what each status means and what may and may not be inferred from it

**§Owns, and §2 *Out of scope*** (review F-12). R-12 constrains what
`goad-check` writes to standard output, which §2 puts out of scope. §Owns
gains, after "the line on standard error that accompanies a failure":

> — and, for `goad-check`, what its report must carry (R-12).

In *Out of scope*, "and the content of standard output, which is an answer to
an invocation rather than a report of how the process ended" gains:

> , save R-12's claim about what `goad-check`'s report carries, which is a statement about each refusal (SPEC-001/R-59) the host reported and not about the report's format

**The P-D paragraph** ("R-7's is the status it does not choose, R-4's is that
same end writing no line, R-6's is the seam §5 names") gains, before "and each
sits in the sentence it qualifies":

> , R-15's is the end it does not choose, as R-7's is, and R-14's is that end writing no line, as R-4's is

### Change 2 — §3: its opening, and P-C

§3's opening sentence loses "including the one §4 does not yet govern", and
reads:

> These hold of every binary this project ships.

In P-C, replace "and `crates/goad-emit/tests/binary/exchange.rs`, which already reads
`goad-emit`'s numbers as a caller sees them even though §4 writes no
requirement for that binary yet." with:

> `crates/goad-emit/tests/binary/exchange.rs` for `goad-emit`, and `crates/goad-check/tests/binary/` for `goad-check`.

### Change 3 — §4: head, and requirements for the two binaries

Replace the paragraph at §4's head ("**Every requirement in this section is
about the host…**") with:

> Requirements are grouped by binary. R-1..R-7 are about the host, `goad`, and none is a statement about another binary.

After R-7, add:

> **`goad-emit`**
>
> | id | requirement | verified by |
> |----|-------------|-------------|
> | R-8 | `goad-emit` MUST exit **0** if, and only if, the host it wrote to accepted the envelope, or the invocation was a question (`--help`, `--version`) and its answer was written. | §7 |
> | R-9 | `goad-emit` MUST exit **1** when a host answered and the answer was a refusal of the envelope. The refusal's reason is carried in the line R-14 requires, and not in the number. | §7 |
> | R-10 | `goad-emit` MUST exit **2** when it got no usable answer: whatever its cause — an argument it cannot use, no configuration or one it cannot read or parse, a configuration naming no socket, a socket nothing answers at, a clock it cannot read, a reply SPEC-003 §6.3 does not admit, or the answer to a question that standard output refused. It MUST NOT distinguish among those causes by status. | §7 |
>
> **`goad-check`**
>
> A **verdict** is what the host reported running this command. `goad-check` has delivered one when it has made every exchange its plan calls for — answering each view, up to its chain bound — and has written its whole report to standard output.
>
> | id | requirement | verified by |
> |----|-------------|-------------|
> | R-11 | `goad-check` MUST exit **0** if, and only if, it delivered a verdict and the host reported nothing on any exchange — no failure, no discarded instruction and no cleanup failure — or the invocation was a question and its answer was written. | §7 |
> | R-12 | `goad-check` MUST exit **1** when it delivered a verdict and the host reported at least one failure, discarded instruction or cleanup failure on any exchange, whichever side SPEC-001/R-59 names for it. Its report MUST carry, for each — a refusal in SPEC-001/R-59's sense — the side and the requirement R-59 names; neither is in the number. | §7 |
> | R-13 | `goad-check` MUST exit **2** when it delivered no verdict: whatever its cause and whenever it arose — an argument it cannot use, a configuration it cannot find, read or parse, an event file it cannot read or that the host's ingress refuses, a clock or runtime it cannot obtain, a report or an answer that standard output refused, or a defect in the checker itself. It MUST NOT distinguish among those causes by status. | §7 |
>
> **`goad-emit` and `goad-check`**
>
> | id | requirement | verified by |
> |----|-------------|-------------|
> | R-14 | Every non-zero exit R-9, R-10, R-12 or R-13 assigns MUST be accompanied by a line on standard error, naming the binary that wrote it and what happened, as the last line that binary writes there. The exception is an end this document does not assign — a signal, or a panic in the binary's own runtime — which §5 bounds for the host and which is bounded here in the same terms. | §7 |
> | R-15 | Neither `goad-emit` nor `goad-check` may exit with a status this document does not define, save for an end it does not choose. Admitting a new status is an amendment here first. | §7 |


### Change 4 — §6: the other binaries' tables

§6's opening sentence becomes "Each binary's statuses, and the whole of what
may be inferred from each." The existing table is headed **The host**. Add:

> **`goad-emit`**
>
> | status | class | what happened | what a reader may infer |
> |---|---|---|---|
> | 0 | **accepted** | The host accepted the envelope, or a question was answered. | The event reached a host that took it. Nothing about what the backend then did. |
> | 1 | **refused** | A host answered, and refused the envelope. | A host answered and refused this envelope; the reason is on standard error. Nothing about whether the same bytes would be accepted later. |
> | 2 | **no usable answer** | No host judged the envelope. | Nothing about the envelope. Whether a host is running is not in the number. |
>
> **`goad-check`**
>
> | status | class | what happened | what a reader may infer |
> |---|---|---|---|
> | 0 | **accepted** | A verdict was delivered, and the host reported nothing. Also a question answered. | The host, running this command with this configuration, would have reported nothing for the requests the checker sent. Nothing about requests it did not send, nor about views the chain bound left unanswered — the report says when it was hit — nor about answering a view when no exchange returned one, which the report also says. |
> | 1 | **refused** | A verdict was delivered, and the host reported something. | The report names each refusal's side and requirement (SPEC-001/R-59). The number does not say whose fault it was. |
> | 2 | **not judged** | No verdict was delivered. | Nothing about the backend. Exchanges may have run before the run stopped; what the host reported of them is not a verdict. |

The "What may not be inferred" paragraph gains:

> For `goad-check`, 0 is not a claim that the backend is correct — only that the host reported nothing of what the checker sent — and 1 is not a claim that the backend is at fault: the side is in the report.

### Change 5 — §7: rows for R-8..R-15

> | R-8, R-9, R-10 | binary tier, `crates/goad-emit/tests/binary/exchange.rs`, each name re-pointed at audit to the test that shipped. R-8: `::an_accepted_envelope_exits_0_and_says_nothing`, `::help_prints_the_usage_block_on_stdout_and_exits_0` and `::version_prints_the_package_version_on_stdout_and_exits_0`; its *only if* for a question is `::an_answer_that_cannot_be_written_exits_2`, which spawns `--help` and `--version` with standard output on a device that refuses every write. R-9: `::a_refusal_exits_1_with_the_reason_token_on_stderr` and `::a_too_soon_refusal_also_shows_retry_after_ms`. R-10: `::a_usage_error_exits_2_before_anything_is_opened`, `::a_path_with_nothing_listening_exits_2_and_names_the_path`, `::a_reply_that_breaches_6_3_exits_2_rather_than_1` and `::an_answer_that_cannot_be_written_exits_2`. What no case reaches: the configuration causes — no path, a file unreadable or unparseable, no `[ingress]` — which are reachable headlessly and have no case, and the clock, which no test controls. So *whatever its cause* is held structurally rather than by that enumeration: every `StartupFault` reaches the status through `startup_failed`, which answers 2 without reading the variant |
> | R-14 | binary tier, and the cases do **not** all hold the same thing. `goad-emit`'s non-zero cases hold a **substring** of standard error and no more — the reason token, the path, the offending argument, the section of SPEC-003 breached — so none holds the binary's name, the whole line, or that it is the last line; those are review: each non-zero path in `main` writes one line through `to_stderr`, whose text `render` begins with `goad-emit: `, and returns. `::an_answer_that_cannot_be_written_exits_2` holds a **prefix**, `goad-emit: `, of the **last** line. `goad-check`'s `::an_unreadable_config_exits_2_and_says_who_spoke` and `::a_reserved_source_event_file_exits_2` each hold a **prefix**, `goad-check: `, of the **last** line of standard error, and not the text past it, which is the operating system's, `toml`'s or SPEC-003's refusal; `::a_discarded_next_check_is_reported_and_exits_1` holds that the last line of standard error begins `goad-check: `. The unassigned end — a signal, a panic — writes no line; no test holds that, for R-15's reason |
> | R-11, R-12, R-13 | binary tier, `crates/goad-check/tests/binary/…`: `::a_conforming_backend_is_accepted_and_exits_0`, `::a_discarded_next_check_is_reported_and_exits_1`, `::a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`, `::an_unreadable_config_exits_2_and_says_who_spoke`, `::a_reserved_source_event_file_exits_2`, `::a_report_that_cannot_be_written_exits_2`. That R-12's report carries the side and the requirement: `::a_refused_view_is_reported_with_its_requirement_and_the_backend_side` and `::an_unspawnable_command_is_reported_against_the_configuration`. What no case reaches: a clock that cannot be read, before the first exchange or during the run, and a runtime that cannot be built, which are headless-unreachable; and a defect in the checker itself, which no cooperating test can produce. *Whatever its cause and whenever it arose* is held structurally: every cause reaches the status through one `ExitCode::from(2)` that reads no cause |
> | R-15 | the compiler and review: each binary's `main` returns an `ExitCode` built from literals, one per class, so a new number cannot appear without a new literal. What neither holds is a panic or a signal, which §5 names as ends this document assigns no status to — the process does not choose those numbers |

### Change 6 — §8: OQ-2 closed

Delete OQ-2. It is answered by Changes 1 and 3.

### Change 7 — §9 References

**Why.** R-12 depends on SPEC-001/R-59, and `goad-emit`'s classes on SPEC-003
§6.3 (review F-12).

**As it will be stated.** The SPEC-001 entry gains:

> ; R-59, the side at fault and the requirement each refusal names, which R-12 requires `goad-check`'s report to carry

The SPEC-003 entry gains:

> ; §6.3, the replies a host may give a writer, which decide between `goad-emit`'s 1 and 2 (R-9, R-10)

---

## POL-001 (the phase gate)

### Change 1 — §Compliance, the command block

**Why.** `examples/` is renamed `exercisers/` (`design-log.md` 2026-09-26,
split by job). The kit ships a TypeScript example that `deno run` would not
typecheck, and memory `deno-run-does-not-typecheck` applies. Both paths go on
one command, so the block keeps its shape and no command is added
(`design-log.md` 2026-09-27, OQ-9).

**As it will be stated.** The fourth line becomes:

```
deno check exercisers/typescript/backend.ts kit/skills/goad-backend/examples/breadcrumbs/backend.ts
```

"The gate is **six commands**" stands. The block is a closed list, and `just
-n check` against this block is what holds it (the exemption in `CLAUDE.md`'s
count rule). The `justfile` mirrors the line, and its "six commands" comment
stands for the same reason.

---

## ADR-003 (the host splits into a workspace of strata) — *design.md OQ-9*

### Change 1 — Decision, the member list

**Why.** The Decision enumerates the members. `goad-check` joins under the
ADR's own rule ("a second binary is a second entry point"). This is an
accumulating reference, which `docs/AGENTS.md` allows an ADR to record without
superseding it. "four at this decision, five since slice 005" is a count, and
it goes stale.

**As it will be stated.** "following ADR-001's strata — four at this decision,
five since slice 005:" becomes:

> following ADR-001's strata:

After the `goad-emit` bullet, add:

> - **`crates/goad-check`** — stratum 3, the `goad-check` binary. Added by slice 012 under the same rule as `goad-emit`: it is a third entry point, it links no renderer, and it drives `goad-shell`'s `Host` headlessly.

The `goad-boundary` bullet's "the other two" (in its own source doc) is not
canon. It is fixed in code, listed in `design.md` §5.2.7.
