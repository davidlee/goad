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

Entries marked *(pending OQ-n)* depend on an open question in `design.md` §6.

---

## SPEC-001 (the host/backend interaction protocol)

No requirement id is renumbered. One is added: **R-59**, the next free id.

### Change 1 — §4 *Failure*: new R-59

**Why.** SPEC-001 §1 promises that a backend author will "know from the error
they get back which side was wrong". No requirement states it, and nothing in
code carried it (`research.md` R-a). The user decided that the claim is canon,
that SPEC-001 owns it because it owns the taxonomy, and that there are four
sides (`design-log.md` 2026-09-26, OQ-2 and OQ-8). The format of whatever
reports the claim is not canon.

**As it will be stated** (a new row after R-47):

> | R-59 | Every refusal the host reports — each distinct error R-44 requires, each discarded instruction R-25 requires, each cleanup failure R-54 requires, and each answer refused under R-32 — MUST name the **side at fault** and the **requirement** of this spec it concerns, and anything that reports such a refusal to a person MUST carry both. The side is one of **backend**, the backend sent or did something this spec refuses; **configuration**, the user's configuration named something the host could not use, such as a command it cannot spawn; **host**, the host's own code failed an obligation this spec places on it; **environment**, the operating system failed the host, or the host observed a condition it cannot attribute to either program. For a backend or configuration side, the requirement is the one broken. For a host or environment side, nobody broke a requirement, and it is the host obligation the refusal left undischarged. The id is a property of the refusal's kind and not of the instance, so one kind names one requirement; where one kind is required by two requirements to be the same error — R-53's `fields` on an alternative, which R-53 requires to be refused as R-50's error is — the requirement is read off what the error names. What a refusal names is fixed here; how it is worded is not. | §7 |

*(pending OQ-1: the "left undischarged" sentence stands only if `design.md`
OQ-1 resolves to (a).)*

### Change 2 — §7: R-59's row

**Why.** The claim needs an instrument. The compiler holds that it is total;
the corpus witnesses that it is correct.

**As it will be stated** (a new row):

> | R-59 | unit, and a witness. `requirement()` and `fault()` are total matches beside each taxonomy, so a new variant does not compile until both are decided: `crates/goad-semantics/src/error.rs::tests::every_protocol_error_names_a_requirement_and_a_side` and its bounds and schedule siblings, and `crates/goad-shell/src/error.rs::tests::every_backend_error_names_a_requirement_and_a_side` and its cleanup and state siblings. That each **answer is right** is witnessed by the corpus, which states requirements independently of the code: `normalize.rs::every_refusal_fixture_names_a_requirement_in_its_own_list` over the `protocol` and `protocol-text` corpora, and `::every_discard_fixture_names_a_requirement_in_its_own_list` over the discards and the `schedule` corpus. The transport, cleanup and state kinds have no fixture, and their answers are held by review of the tables. That a reporter carries both is `crates/goad-check/tests/binary/…::a_refused_view_is_reported_with_its_requirement_and_the_backend_side` and `::an_unspawnable_command_is_reported_against_the_configuration` |

### Change 3 — §7: R-56's row

**Why.** Two facts in the row change. The one place the host names a kind
moves to stratum 1 (`design-log.md` 2026-09-26, OQ-4). The backend tolerance
clause, held today by "review, not a test", becomes testable: `goad-check`
sends an unrecognised host kind and reports a backend that fails on it.

**As it will be stated.** Replace the sentence beginning "unit, at the one
place the host names a kind:" through "…and nothing else." with:

> unit, at the one place the host names a kind: `crates/goad-semantics/src/protocol/canonical.rs::tests::a_scheduled_stimulus_names_itself_scheduled` and `::a_scheduled_stimulus_s_event_carries_the_three_normative_fields`, the second asserting `source`, `kind`, `timestamp` and payload together, against `Stimulus::kind`, which returns exactly `"startup"`, `"requested"` and `"scheduled"` and nothing else.

Replace the sentence beginning "The **tolerance** clause is an obligation on
backends" with:

> The **tolerance** clause is an obligation on backends, which no host test can observe of a backend it does not ship; it is tested of any backend a person points the checker at, since `goad-check` originates an `evaluate` whose kind is none of the three and reports a backend that fails on it against this requirement: `crates/goad-check/tests/binary/…::a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`, with `::the_probe_kind_is_none_of_the_host_s_own` holding that the probe is an unrecognised kind and not one of the three. Each backend the kit ships is held to it in the gate by `crates/goad-check/tests/kit/…::each_shipped_example_is_accepted_by_the_checker`.

In the emission-half sentence, replace "`Stimulus::kind` is the one place the
host names its own event, `crates/goad/src/wire.rs` the one place it is
written" with:

> `Stimulus::kind` (`crates/goad-semantics/src/protocol/canonical.rs`) is the one place the host names its own event, and `Stimulus::event` the one place it is built

### Change 4 — §7: R-57's row

**Why.** The single site moves to stratum 1 so that the checker's own respond
obeys R-57 without restating it (`design-log.md` 2026-09-26, OQ-4). The row's
argument, "one site is the requirement's own structure", now holds across the
workspace rather than across one crate.

**As it will be stated.** Replace the opening through "…in both directions."
with:

> unit, at the **single site** a submitted value is written: `Submitted::to_json` (`crates/goad-semantics/src/protocol/canonical.rs`), a total match over `Submitted`, whose variants are `FieldKind`'s one for one, tested by `canonical.rs::tests::every_submitted_kind_writes_the_json_type_r57_names`. Both writers go through it: the renderer's `draft::submitted` (`crates/goad/src/draft.rs`) projects its widget state onto `Submitted` and decides no type — `draft.rs::tests::the_projection_to_submitted_is_the_identity_on_each_kind` — and the checker builds `Submitted` directly.

Replace the closing sentence ("The site that must change when the *protocol*
grows a sixth kind is not this match…") with:

> The site that must change when the *protocol* grows a sixth kind is this match's type, `Submitted`, which must gain a variant before `to_json` compiles — and, for the renderer, the `FieldKind` arm in `view_model.rs::drawn_form`, which must sort the new kind into drawn or `Undrawn::FieldForm`.

*(pending OQ-2: if (a), add "and what a field nobody touched submits is
`Submitted::as_drawn`, which both writers share".)*

### Change 5 — §7: the paragraphs about rows held by review

**Why.** The intro says "Five rows are held by **review**", and the closing
paragraph says "Five rows … R-56 is a sixth only in part". Both are counts
that nothing re-reads, and Change 3 makes the second one false: R-56's
backend clause is now tested.

**As it will be stated.** The intro sentence becomes:

> Some rows are held by **review** rather than by a test, and each says so and why.

The closing paragraph becomes:

> Nothing here is marked unverified. The rows for R-9/R-19, R-18, R-20, R-30 and R-49 are held by review rather than by a test, each for a reason stated in the row: the subject is a property of the source text or of the contract, not a behaviour anything can execute. R-56's emission half is held by review for the reason its row gives. R-49 constrains the other side of the seam, and no checker can observe a side effect a backend did not report.

---

## SPEC-004 (process exit status)

**Why.** The checker is a new binary, and SPEC-004 owns every binary's status
(AC-4). The user decided to govern `goad-emit` in the same change
(`design-log.md` 2026-09-26, OQ-5), which closes this spec's OQ-2. §3 is
unchanged; it already binds every binary. `design.md` §5.2.5 sets out the
checker's classes.

### Change 1 — §2 Boundaries

Replace the first two bullets ("`goad-emit` is nominally owned and not yet
governed…" and "`goad-emit`'s classes are not the host's classes…") with:

> - **Three binaries, three cuts.** §4 governs each binary this project ships — the host, `goad`; `goad-emit`; and `goad-check` — under its own heading. They share §3's principles and not one another's classes: the host's cut is phase against its event loop; `goad-emit` reports a refusal **as an answer** — the host it wrote to considered the envelope and said no — and a host that never answered has none to give; `goad-check` reports **a judgement** — the host's own code refused something the backend did — and a checker that never reached a backend has none to give. A new binary is an append to §4 and a table in §6.

Keep the SPEC-001 §2 bullet. It still explains why the `goad emit` command
line is this document's to own.

### Change 2 — §3 P-C

Replace "and `crates/goad-emit/tests/binary/exchange.rs`, which already reads
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
> | R-9 | `goad-emit` MUST exit **1** when a host answered and the answer was a refusal of the envelope. The refusal's reason is carried in the line R-11 requires, and not in the number. | §7 |
> | R-10 | `goad-emit` MUST exit **2** when it got no usable answer: whatever its cause — an argument it cannot use, no configuration or one it cannot read or parse, a configuration naming no socket, a socket nothing answers at, a clock it cannot read, or a reply SPEC-003 §6.3 does not admit. It MUST NOT distinguish among those causes by status. | §7 |
>
> **`goad-check`**
>
> | id | requirement | verified by |
> |----|-------------|-------------|
> | R-11 | `goad-check` MUST exit **0** if, and only if, it made every exchange it planned and the host reported nothing on any of them — no failure, no discarded instruction and no cleanup failure — or the invocation was a question and its answer was written. | §7 |
> | R-12 | `goad-check` MUST exit **1** when it made every exchange it planned and the host reported at least one refusal, discarded instruction or cleanup failure on any of them, whichever side SPEC-001/R-59 names for it. The side and the requirement are in the report, not in the number. | §7 |
> | R-13 | `goad-check` MUST exit **2** when it did not reach its first exchange: whatever its cause — an argument it cannot use, a configuration it cannot find, read or parse, an event file it cannot read or that the host's ingress refuses, or a clock or runtime it cannot obtain. It MUST NOT distinguish among those causes by status. | §7 |
>
> **Both binaries**
>
> | id | requirement | verified by |
> |----|-------------|-------------|
> | R-14 | Every non-zero exit R-9, R-10, R-12 or R-13 assigns MUST be accompanied by a line on standard error, naming the binary that wrote it and what happened, as the last line that binary writes there. The exception is an end this document does not assign — a signal, or a panic in the binary's own runtime — which §5 bounds for the host and which is bounded here in the same terms. | §7 |
> | R-15 | Neither binary may exit with a status this document does not define, save for an end it does not choose. Admitting a new status is an amendment here first. | §7 |

*(pending OQ-5: R-11 and R-12 include cleanup failures; if OQ-5 resolves to 0,
both clauses drop "or cleanup failure".)*

### Change 4 — §6: the other binaries' tables

§6's opening sentence becomes "Each binary's statuses, and the whole of what
may be inferred from each." The existing table is headed **The host**. Add:

> **`goad-emit`**
>
> | status | class | what happened | what a reader may infer |
> |---|---|---|---|
> | 0 | **accepted** | The host accepted the envelope, or a question was answered. | The event reached a host that took it. Nothing about what the backend then did. |
> | 1 | **refused** | A host answered, and refused the envelope. | The envelope was judged and found wanting; the reason is on standard error. Sending the same bytes again will be refused again unless the host's state changed. |
> | 2 | **no usable answer** | No host judged the envelope. | Nothing about the envelope. Whether a host is running is not in the number. |
>
> **`goad-check`**
>
> | status | class | what happened | what a reader may infer |
> |---|---|---|---|
> | 0 | **accepted** | Every planned exchange ran and the host reported nothing. | The host, running this command with this configuration, would have reported nothing for the requests the checker sent. Nothing about requests it did not send. |
> | 1 | **refused** | Every planned exchange ran and the host reported something. | The report names each refusal's side and requirement. The number does not say whose fault it was. |
> | 2 | **not judged** | No exchange was made. | Nothing about the backend. |

The "What may not be inferred" paragraph gains:

> For `goad-check`, 0 is not a claim that the backend is correct — only that nothing the checker sent was refused — and 1 is not a claim that the backend is at fault: the side is in the report.

### Change 5 — §7: rows for R-8..R-15

> | R-8, R-9, R-10 | binary tier, `crates/goad-emit/tests/binary/exchange.rs`: the cases there that read 0, 1 and 2 as a caller sees them — re-pointed at audit to the test names that shipped. What no case reaches is named in the row as R-3's row names it |
> | R-14 | binary tier, each binary's cases that read standard error on a non-zero exit |
> | R-11, R-12, R-13 | binary tier, `crates/goad-check/tests/binary/…`: `::a_conforming_backend_is_accepted_and_exits_0`, `::a_discarded_next_check_is_reported_and_exits_1`, `::a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`, `::an_unreadable_config_exits_2_and_says_who_spoke`, `::a_reserved_source_event_file_exits_2` |
> | R-15 | the compiler and review: each binary's `main` returns an `ExitCode` built from literals, one per class |

### Change 6 — §8: OQ-2 closed

Delete OQ-2. It is answered by Changes 1 and 3.

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

## ADR-003 (the host splits into a workspace of strata) — *for confirmation (design.md OQ-9)*

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
