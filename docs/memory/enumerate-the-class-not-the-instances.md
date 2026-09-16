# An enumeration written to close a class will be short; write the class and the grep

Learned in slice 007's code review, rounds 2 through 5. Four successive rounds
each found an unheld "a path no other case will collide with" helper, repaired
it, and wrote down the list of helpers they had found. **Three of those four
lists were short within the round that wrote them.**

- F-5 repaired two cases sharing an invocation log, and said so.
- F-9 found a second helper making the same promise. The repair's doc then
  claimed to be "behind **every**" such promise.
- F-11 falsified "every" — two more helpers, one in the same test binary. The
  repair enumerated "the three `socket_path` helpers" by path, explicitly *"so a
  fifth helper has a worked example rather than a principle"*.
- F-17 falsified "three": a fourth, found by `grep -rn 'fn socket_path'`.
- F-22 falsified "five": a **sixth**, invisible to that grep — `config_home` in
  `crates/goad-emit/src/main.rs`, a `#[cfg(test)]` helper inside a `src/` file,
  minting a directory rather than a socket, and destructive in the same way
  (it writes or removes `config.toml` as it hands the path out).

Each list was correct when written and wrong one round later, and each round had
just been raised about an incomplete enumeration.

**The defect is not the count. It is enumerating instances at all.** A list of
instances is short the moment a member of the class does not resemble the
others, and the member that does not resemble the others is exactly the one
nobody has checked. The repair that finally held was to lead with the class and
the command that finds it:

> A helper of this class mints a path under `std::env::temp_dir()` qualified by
> the process id, promises in prose that it is the caller's own, and clears or
> overwrites what is there when it hands the path out. It need not be called
> `socket_path`, need not mint a socket, and need not live under `tests/`.
>
> ```text
> grep -rn 'process::id()' --include=*.rs . | grep -v '^./target'
> ```

with each hit falling into one of three named buckets, so the exclusions are a
rule a reader can apply rather than a judgement they must repeat.

**How to apply.** When a repair's doc is about to say "the three X", "all four",
or "every Y in this repository", stop: that sentence is a claim nothing checks,
and it is the same defect as
[[a-count-in-a-comment-is-a-claim-nothing-checks]]. Write what makes something
an X, and the command that enumerates them. If the class genuinely cannot be
expressed as a grep, that is worth knowing before the list is written down.

See also [[a-repair-sweep-misses-the-binding-site]] — the same shape one level
down, and [[a-test-rule-binds-to-a-defect-not-a-surface]].
