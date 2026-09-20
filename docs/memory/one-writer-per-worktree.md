# One writer per worktree, and never rewrite history to tidy up

Learned in slice 007, when two concurrent agents and a `git reset` lost a
commit; reaffirmed throughout slice 009's audit.

## The fact

Two agents editing one checkout will eventually both run a `git` command, and
one of them will be a command that moves refs or the index. When that happens,
the other's work disappears with no error.

**"Done" is not the same as "idle."** An agent that has reported its result may
still have a shell open, a build running, or a file half-written. Treat an agent
as holding the tree until you have seen it exit.

## How to apply

- **One agent writes per worktree.** If work must run in parallel, give each
  agent its own worktree — and see `subagent-worktrees-can-be-stale.md`, because
  the worktree it gets is probably not at the commit you think.
- **Commit before spawning.** A worktree branches from a commit, so uncommitted
  work in the main tree is invisible to the agent. If the agent is meant to build
  on it, it has to be a commit first.
- **Merge back with `git merge --ff-only`.** It moves a branch pointer over
  commits it already contains and discards nothing. Verify afterwards that your
  own uncommitted work is still there.
- **Never** `git stash`, `git checkout`, `git checkout-index`, or `git reset` in
  this repository. To restore a file, copy it to a scratchpad before you change
  it and copy it back — and then verify with `git status --short` that the tree
  is clean. An injection runner that reverts with `git checkout -- <path>`
  against an uncommitted tree reverts a whole phase; slice 009's prototype did
  exactly that.
- **Never rewrite history.** No rebase, no amend, no force. A slice's record is
  the commits as they happened, and half the audit's value is being able to read
  what a commit actually changed.

Related: `subagent-worktrees-can-be-stale.md`,
`git-stash-forbidden-recover-read-only.md`,
`an-injection-that-reorders-a-fallback-is-not-an-injection.md`.
