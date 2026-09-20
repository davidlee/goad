# A subagent's worktree arrives stale, and the fix is a fast-forward

Four times in slice 009's audit, at 25, 38, 42 and 43 commits behind. Twice at
the **same** commit, `f352124`. This is not a hazard to watch for; it is how
worktree provisioning behaves here.

## The fact

An agent spawned with its own git worktree does **not** reliably start at the
commit the parent is on. It starts at some earlier commit — in this project,
usually a specific stale one — on its own branch.

An agent that does not check will read a tree one or more sessions old and
report on it with complete confidence. Slice 009's round-3 reviewer would have
reviewed a tree two sessions old; its round-4 reviewer, a tree 42 commits old.
Nothing in the output would have looked wrong.

## How to apply

Put this in **every** subagent brief that touches the repository, as Step 0,
before any reading:

> Run `git log --oneline -1`. The commit you must work at is `<sha>`.
>
> - If HEAD is already `<sha>` — proceed.
> - Otherwise check `git merge-base --is-ancestor <your HEAD> <sha>` and
>   `git status --short`. **If your HEAD is a clean strict ancestor and your
>   tree is clean**, run `git merge --ff-only <sha>` and proceed. That is a
>   fast-forward, not a checkout or a reset, and it is allowed.
> - If it is **not** an ancestor, or your tree is dirty, **stop and report**.
>
> Say in your report which of these happened.

Two details that cost time before they were written down:

- **"Stop and report" alone is not enough.** The first citation agent did
  exactly that, correctly — and its worktree was then **auto-removed** when it
  exited without changes, so the recovery had to start over. Give the agent the
  recovery, not just the tripwire.
- **`git merge --ff-only` is the right recovery** and does not breach this
  project's standing ban on `git stash`, `git checkout`, `git checkout-index`
  and `git reset`. It moves a branch pointer forward over commits it already
  contains; it discards nothing.

And for the parent: **commit before spawning.** An agent's worktree branches
from a commit, so uncommitted work in the main tree is invisible to it. If the
agent is meant to build on that work, it has to exist as a commit first.

## Why it matters more than it looks

The failure is silent and the output is confident. There is no error, no
conflict, and no missing file — just a coherent, well-argued review of code
that no longer exists. Every downstream artefact then inherits it: the ledger,
the dispositions, the closing argument.

One writer per tree, always: two agents editing one worktree while either runs
a `git` command is how a commit gets lost.

Related: `git-stash-forbidden-recover-read-only.md`,
`a-code-reviews-findings-go-stale.md`.
