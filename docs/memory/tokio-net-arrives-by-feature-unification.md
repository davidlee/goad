# `tokio::net` is reachable from `crates/goad`'s tests only by feature unification

Noticed at slice 004, PHASE-04.

## The fact

`crates/goad/Cargo.toml` declares `tokio` with `rt-multi-thread` and `sync`.
`net` is available anyway, because `goad-shell` enables it and cargo unifies
features across the graph.

So code in `crates/goad` that names `tokio::net` compiles today and breaks the
day `goad-shell` stops needing `net` — for a reason nothing in `crates/goad`
states.

## How to apply

For a socket a **test** opens, `std::os::unix::net` on
`tokio::task::spawn_blocking` needs no feature at all, and keeps the blocking
call off the thread the loop runs on.

Reach for the manifest only when **production** code needs the feature. Adding
a feature to a manifest to legitimise a test's convenience buys a permanent
dependency edge for a temporary reason.

The general shape: a crate compiling against a feature it does not declare is
borrowing a sibling's decision, and the borrow is invisible at the borrowing
crate.
