# Design — Slice NNN: {title}

<!-- The *current* design, not its history. Revision chronology, review
     findings, and dispositions live in `design-log.md`.
     Reference forms: canon by id (`SPEC-003 §4`, `ADR-007`, `POL-002`);
     doc-local refs bare — OQ-1 (§6), D1 (§7), R1 (§8). Ids are immutable. -->

## 1. Design problem

<!-- In plain language: what changes, why it matters, and the boundary of this
     design. The reader should not need the slice doc or review history to
     orient themselves. -->

## 2. Current state

<!-- How it works today, cited. Cite `research.md` rather than restating it. -->

## 3. Forces & constraints

<!-- Canon that binds, technical limits, cost, timing, prior commitments. -->

## 4. Guiding principles

<!-- The few rules that settle the arguments below. Two or three, not ten. -->

## 5. Proposed design

### 5.1 System model

<!-- The load-bearing structure and who owns what. Prefer a Mermaid
     context/container/component diagram once there is more than one part. -->

### 5.2 Interfaces & contracts

<!-- Signatures, wire shapes, CLI surfaces, error cases. Exact names. -->

### 5.3 Data, state & ownership

<!-- What is stored, where it lives, who may write it, what is derived and
     therefore disposable. -->

### 5.4 Lifecycle & dynamics

<!-- Behaviour over time: startup, failure, retry, concurrency. Use a Mermaid
     sequence or state diagram when behaviour crosses a boundary or branches
     materially. -->

### 5.5 Invariants, assumptions & edge cases

<!-- What must always hold; what is assumed without proof; what happens at the
     edges. Each assumption is a place the design can break. -->

## 6. Open questions

<!-- Carried from `slice-nnn.md` OQ-N plus any raised here. Nothing may remain
     open at design acceptance without an explicit user decision to defer. -->

## 7. Decisions, rationale & alternatives

<!-- D1, D2… One row per decision that a later reader might otherwise reverse
     by accident: what was chosen, what was rejected, and why. -->

## 8. Risks & mitigations

<!-- R1, R2… Risk, likelihood/impact, mitigation, and the signal that tells you
     it is happening. -->

## 9. Validation

<!-- How the design will be shown correct: the tests, checks, and observations
     the plan must produce. Feeds the plan's verification criteria. -->

## 10. Canon impact

<!-- Specs, policies, ADRs this design amends, adds, or contradicts. Each entry
     is a debt reconciliation must settle. -->
