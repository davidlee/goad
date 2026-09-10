# A repair sweep finds the prose and misses the binding site

Learned at slice 004's design review, where three of four rounds yielded this
same class.

## The fact

A repair lands correctly where it was pointed, and is not carried to the
artefact that states the same thing **normatively**. Both of round 3's contests
were this, and both times the missed site was the more binding one: a spec
requirement (a MUST), and a sequence diagram in which position is time.

Prose siblings get swept. A MUST and a picture do not — a text search for the
repaired wording does not match either, and neither reads like the thing that
was fixed.

## The rule

When dispositioning a `doc-wrong`, **name the most binding artefact by hand in
the repair brief.** Do not rely on the repairer sweeping for it.

## How to apply

Before writing the disposition, ask: where else is this same claim stated, and
which of those statements is the one another document would be checked against?
Requirements tables, sequence diagrams, and interface blocks are the three that
a prose sweep reliably misses.

Related: `cite-requirements-not-finding-ids.md` — the binding artefact is
usually the one worth citing, for the same reason.
