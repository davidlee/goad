# negative-control-must-compile

**This is a pointer.** The lesson lives in
[`a-negative-control-that-does-not-compile.md`](a-negative-control-that-does-not-compile.md).

Slice 007's and slice 009's artefacts cite it under this shorter name — eight
references across `review-code.md`, `notes.md` and `prototype-notes.md` — and
those files are append-only, so the citations cannot be corrected in place. This
file exists so they resolve.

The one-line version: **a control that fails to compile greps identically to one
that passes.** Slice 009's first attempt at an injection removed a guard
outright and failed on `unused import: Model`; rewritten as *compute and
ignore*, it went red for the right reason.
