# `Path` equality ignores an interior `.`; a claim about bytes compares `OsStr`

Learned at slice 015, PHASE-01 (`docs/slices/015/notes.md`, PHASE-01
*Findings*), writing the cases that pin how a relative path is joined onto a
configuration's directory.

## The fact

`Path`'s `==` compares **components**, and `Path::components` drops a `.`
that is not the first component:

```rust
assert_eq!(Path::new("/a/./b"), Path::new("/a/b")); // passes
assert_ne!(Path::new("/a/./b").as_os_str(), OsStr::new("/a/b")); // passes too
```

So a test that asserts *which* bytes a join produced cannot use `PathBuf`
equality. Slice 015's first `config.rs` join cases did, and the mutation that
kept the `.` (`directory.join(path)` in place of the component-wise join) left
them **green**. Comparing `as_os_str()` against `OsStr::new(..)` reds them.

It was found by running the mutation, not by reading the test: the assertion
reads as exact.

## How to apply

- When the bytes are the claim — what a child process receives as its name,
  what a socket is bound at, what a spec says the join produces — compare
  `as_os_str()`. SPEC-001/R-60 and SPEC-003/R-17 state the join byte for byte
  (each `.` dropped, each `..` kept), so their cases must.
- When the claim is *the same file*, `Path` equality is right, and so is
  `canonicalize` on both sides. Choose deliberately; the two read alike.
- `Path` equality does **not** fold `..`, and neither does `Path::components`:
  only `.` is lost. A `..` mismatch is caught either way.

Related: `a-green-test-can-assert-a-proxy.md`,
`a-negative-control-that-does-not-compile.md`.
