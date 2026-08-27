# Canon delta — Slice 001

Changes this slice makes to canon that **already exists**. New canon is drafted
in `draft-spec.md`, not here.

One entry per affected document: the document, the section, the change as it will
be stated, and why. Applied during audit and reconciliation with explicit user
endorsement, and recorded in `audit.md`'s Reconciliation table.

---

## CD-1 — ADR-001, "Verification"

**Document:** `docs/adr/001-one-way-strata.md`
**Section:** Verification
**Kind:** record accuracy. The **decision** is untouched.

### Why

ADR-001's Verification section says the one-way rule is checked "by inspection
during review, and by the placement discipline recorded in each slice's design",
and concludes that "until [the strata become separate crates], the honest answer
is that this is a review gate, not a build gate."

Slice 001 adds two mechanisms, and they are not the same strength.

AC-15 is a test asserting that no file under `src/semantics/` names
`crate::shell`, `crate::bin` or `tokio`. It catches the common upward reference,
not the class: it cannot see a downward type leak, a re-export that flattens the
boundary, or `std::fs` appearing in stratum 1.

The second is stronger and was added after F-51. tokio is an **optional**
dependency behind a `shell` feature, so `cargo test --no-default-features`
builds and runs stratum 1 with no runtime in its dependency graph — Cargo's own
resolution, not a grep. That half of ADR-001's Decision therefore *is* a build
gate now, in a single crate, which the ADR assumed impossible before the
workspace split.

So the ADR's conclusion is now half right: the **dependency-graph** claim is
mechanically verified; the **direction** claim is still a review gate. Stating
that distinction is the delta.

Per `docs/AGENTS.md`, an ADR's decision is fixed while its record is kept
accurate as consequences are learned. This is that case, which is why it is a
delta rather than a supersession.

### Change as it will be stated

Replace the first paragraph of ADR-001's Verification section with:

> This decision makes two claims, and since slice 001 they are verified
> differently.
>
> **Stratum 1's dependency graph** is a build gate. The async runtime is an
> optional dependency behind a `shell` feature, so
> `cargo test --no-default-features` builds and tests the semantic core with no
> runtime present, and fails if that stops being true. `cargo tree
> --no-default-features` is the diagnostic. This holds inside a single crate and
> does not wait for the workspace split.
>
> **The direction of dependencies** remains a review gate: inspection during
> review, the placement discipline recorded in each slice's design, and a test
> asserting that no file under `src/semantics/` names `crate::shell`,
> `crate::bin` or `tokio`. That test catches the common upward reference, not the
> class — it cannot see a downward type leak, a re-export that flattens the
> boundary, or I/O reaching stratum 1 through `std` rather than through a named
> crate. Treat a green run as the absence of the obvious violation, not as
> compliance.

Leave the second paragraph ("If the strata ever become separate crates …")
unchanged: it is still exactly true, and it is now the remaining gap rather than
the whole of it.

### Verified by

`docs/slices/001/design.md` §5.1 (the manifest, and what AC-15 checks versus
what the feature gate checks), §5.5 A2 (the remaining limit stated as an
assumption), R1 in §8, and §9's verification commands. The feature gate was
verified by building it: `cargo tree --no-default-features` contains no tokio
node and `cargo test --no-default-features` compiles and runs.

---

## CD-2 — ADR-002, "Decision"

**Document:** `docs/adr/002-single-crate-until-triggered.md`
**Section:** Decision (the T1 bullet), and the rejected alternative about a
Cargo feature
**Kind:** record accuracy. The trigger set, the standing position and the
decision are all untouched.

### Why

ADR-002's T1 is "a dependency is required that stratum 1 must not need in order
to build", annotated "Slint is the first such dependency, expected in slice
002."

That annotation is now wrong by one slice. tokio is a dependency stratum 1 must
not need, and it arrives in slice 001. It does not fire T1 only because slice 001
makes it **optional** behind a `shell` feature (F-51, D49), so nothing stratum 1
must not need is required *in order to build* stratum 1 — which is what T1
actually says.

This matters beyond bookkeeping, because a reader checking triggers in a later
slice needs to know that "optional dependency" is an available answer to T1 and
what its limits are. It works for a runtime. ADR-002's own rejected alternative
already explains why it does not work for the renderer: a Slint build-dependency
with a conditional `build.rs` keeps stratum 1 renderer-free "only by vigilance".
So T1 is still expected to fire in slice 002, for a reason the ADR already gives.

### Change as it will be stated

Replace T1's annotation:

> - **T1** — a dependency is required that stratum 1 must not need in order to
>   build. A dependency that can be made *optional*, behind a feature that gates
>   the stratum needing it, does not fire T1: slice 001 admits the async runtime
>   that way, and verifies it with `cargo test --no-default-features`. Slint is
>   expected to fire T1 in slice 002, because a build-dependency with a
>   conditional `build.rs` cannot be gated as cleanly — see the rejected
>   alternatives below.

No other change. In particular the rejected alternative "a single crate
permanently, with the renderer behind a Cargo feature" stands as written: slice
001 uses a feature for one dependency in one slice, which is not a proposal to
avoid the split.

### Verified by

`docs/slices/001/design.md` §3 (the trigger analysis, including the false earlier
answer and why it was false) and §5.1 (the manifest).

---

## Considered and no delta owed
- **`docs/brief.md`.** Not canon by `docs/AGENTS.md`'s definition — it is intent.
  Three of its ambiguities were resolved by choice in this slice (`design.md`
  §5.5 A4); those are recorded as decisions, and if any should bind future
  slices it becomes a spec requirement or an ADR, not an edit to the brief.
- **Root `AGENTS.md`.** A deliverable of this slice (AC-10), not canon.
