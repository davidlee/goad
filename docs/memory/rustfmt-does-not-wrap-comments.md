# rustfmt does not wrap comments, so the gate never sees a doc line's width

Learned at slice 015's code review (`docs/slices/015/review-code.md` F-10,
F-11).

## The fact

`just check`'s format step is `cargo fmt --all --check`, and rustfmt reflows
code, not comments: `wrap_comments` is an unstable option, and `rustfmt.toml`
sets only `tab_spaces`. A doc comment edited in place and not reflowed — a
117-column line, or a paragraph that ends mid-sentence on a short line and
resumes on the next — passes the gate.

Slice 015 shows how it recurs. Round 1 found three over-long doc lines the
slice had written (F-10) and repaired them with a class check over the slice's
diff. Round 2 found that the **repairs** for a different finding had written a
fourth, and left two paragraphs half-reflowed (F-11): the class check had
covered the lines that existed before the repairs, not the repairs' own.

## How to apply

- After editing a doc comment, reflow its paragraph to the file's own wrap
  (most of this repository wraps near 80); do not stop at the edited line.
- The class check is a script over the diff, run **after the last repair**,
  from the slice's base:

  ```text
  git diff -U1 <base> -- '*.rs'
  ```

  then flag added comment lines past the file's wrap, and an added comment
  line well short of it followed by a non-blank comment line (a half-reflowed
  paragraph). Judge each against the file: some files wrap a little wider.
- A repair is new text. Whatever check found the defect runs again over the
  repair — the same lesson as `cite-by-symbol-not-line-number.md`, where the
  repairing commit re-created the class.

Related: `enumerate-the-class-not-the-instances.md`.
