# A repair can be wrong about what it holds, and its own Response can name the case that would have caught it

Slice 009's audit, `review-code.md` F-B9 → F-C6. The second of two times in one
audit that a repair closed a finding on paper and left the defect live.

## The fact

F-B9 said a diagnostics repeater was handed a fresh model on every present, so
every line element was destroyed and rebuilt. The repair retained the model —
correctly addressing the path the finding *described* — and left the write
unguarded, which left a **second** path to the same defect wide open.

The repair's own Response was honest about the residue. It said, in as many
words, that it had not been injection-passed, and it **named the case that
would have settled it**: an `init` counter on the repeated element, read across
three presents. It then explained why it had not written that case.

Round 3 wrote the case. It caught the defect on the first run.

**The residue the Response declined to close was the defect.**

## Why

A repair is written by whoever understands the finding best, which is also
whoever is most convinced the mechanism they found is the whole mechanism. The
Response is that conviction in prose, and prose does not have to be exhaustive
to be persuasive — it only has to account for the evidence already on the table.

The named-but-unwritten case is the tell. Naming it means the author saw the
question; declining to write it means the author answered it by reasoning. A
mechanism argument that sounds complete and a mechanism argument that *is*
complete read identically.

## How to apply

- **A Response that says "this was not injection-passed" is a finding.** Treat
  it as an open item, not as a disclosure that discharges itself. Honesty about
  a gap is not the same as closing it.
- **If a Response names the case it did not write, write that case.** It costs
  less than the round that finds out. Both times this happened in slice 009, the
  named case was cheap and decisive.
- **Review repairs, not only original code.** Slice 009 ran four rounds and
  every round after the first reviewed the previous round's repairs, which
  nobody else had looked at. Rounds 2 and 3 each found a major *in a repair*.
- When a finding describes a mechanism, ask what **else** produces the same
  observable. Two independent paths to one symptom is the shape here: pointer
  identity and a mutation that never consults the pointer.

Related: `a-green-test-can-assert-a-proxy.md`,
`a-negative-control-that-does-not-compile.md`,
`a-present-destroys-the-widget-it-writes.md` for the mechanism itself.
