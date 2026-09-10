# A second tokio runtime, entered around a spawn and then shut down, is how a test reaches a dead background task

Found at slice 004, PHASE-04/S-5, while looking for a way to test the
"the task feeding this channel is gone" state without a production API for it.

## The fact

A constructor that spawns a background task spawns it onto whatever runtime is
**entered** when it is called. So:

```rust
let second = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
let handle = { let _entered = second.enter(); bind(&path)? };
second.shutdown_background(); // drops the task, and the channel's only sender
```

The task is gone, the channel's sender with it, and the receiving arm now
yields `None` and parks — the state under test, reached with no panic and no
test-only constructor.

`shutdown_background` rather than `drop`: dropping a `Runtime` inside an async
context panics.

## Why it is worth knowing

It kept a public type from growing a test-only constructor whose only purpose
was to express a state the production code can reach on its own. The same shape
reaches any "the task that feeds this channel is gone" condition.
