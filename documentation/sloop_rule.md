# The Rust rules, summarized

## 1. Keep control flow simple and bounded
No recursion, and every loop has a provable upper limit stored in a named constant. This means a bug can never turn into an endless loop or a stack overflow.
```rust
const RETRY_MAX: u32 = 5;
for _attempt in 0..RETRY_MAX {
    if try_send().is_ok() { break; }
}
```

## 2. Put a limit on everything
Queue sizes, batch sizes, message sizes, retries, and connections each get a named constant. Anything that can grow from outside input must have a ceiling. Test at `LIMIT - 1`, `LIMIT`, and `LIMIT + 1`.
```rust
const BATCH_MAX: usize = 8_192;
if batch.len() > BATCH_MAX { return Err(Error::TooLarge); }
```

## 3. Keep functions short
Aim for about 70 lines or fewer. Let the caller make the decisions (`if`) and let small pure helpers do the repetitive work (`for`). Small pure functions are also the easiest to test.
```rust
fn handle(batch: &[Item]) {
    if batch.is_empty() { return; }          // parent decides
    for item in batch { process(item); }     // helper does the work
}
```

## 4. Allocate at startup, not at steady state
Create buffers once during initialization, then reuse them. The hot path should never ask the allocator for memory.
```rust
let mut buf: Vec<u8> = Vec::with_capacity(BATCH_MAX); // once, at startup
buf.clear();                                          // reuse, never grow
```

## 5. Do a performance sketch first
Before coding, estimate the cost of network, disk, memory, and CPU, for both bandwidth and latency, and optimize the slowest one first. Batch by default, and keep a benchmark baseline in CI.
```text
10,000 writes/s, fsync ≈ 1 ms  ->  1 fsync per write is too slow
-> batch 100 writes per fsync  ->  100 fsyncs/s, fine
```

## 6. Be explicit about types
Use fixed-size integers, wrap IDs and units in newtypes so they can't be mixed up, and avoid lossy `as` casts.
```rust
struct UserId(u64);
struct OrderId(u64);
let small = u32::try_from(big)?;   // not `big as u32`
```

## 7. Assert aggressively, in pairs
Check conditions before and after important operations, so a bug is caught right where it happens. Use `assert!` (always on) for real invariants, and `const` assertions for anything known at compile time.
```rust
assert!(self.len <= self.capacity());   // before
self.len += 1;
assert!(self.len <= self.capacity());   // after
const _: () = assert!(size_of::<Header>() == 64);
```

## 8. Make errors explicit and exhaustive
Expected failures are values (`Result`), violated invariants are panics. Match on your own enums without a `_` wildcard, so adding a variant breaks the build everywhere it matters. Parse raw input into validated types once, at the edge.
```rust
enum PushError { Full, TooLarge }
match err {
    PushError::Full => retry_later(),
    PushError::TooLarge => reject(),   // no `_ =>`
}
```

## 9. Minimize and isolate `unsafe`
Forbid it by default. Where it's truly needed, keep it in a tiny module behind a safe API, with a comment stating why it's sound.
```rust
#![forbid(unsafe_code)]   // default for the crate
// SAFETY: index was checked against len() two lines above
unsafe { *slice.get_unchecked(i) }
```

## 10. Zero warnings, strictest tooling
Turn on the strict lints and make CI fail on any warning. Pin the toolchain so builds are reproducible.
```rust
#![deny(unused_must_use, clippy::unwrap_used)]
```
```bash
cargo clippy --all-targets -- -D warnings
```

## 11. Make time, randomness, and I/O injectable
Hide the clock, the random number generator, the network, and the disk behind traits. The core logic then becomes deterministic and testable with fakes.
```rust
trait Clock { fn now(&self) -> Instant; }
fn is_expired(clock: &impl Clock, deadline: Instant) -> bool {
    clock.now() > deadline
}
```

## 12. Test with simulation, fuzzing, and properties
Don't rely only on hand-picked examples. Run random operations against a trusted model with a fixed seed, so any failure can be replayed exactly. Test failure paths at least as much as success paths.
```rust
for seed in 1..=64 {
    let mut rng = XorShift64(seed);
    // run random ops on `ring` and `VecDeque`, compare results
    assert_eq!(real.pop(), model.pop_front().ok_or(Empty), "seed={seed}");
}
```

## 13. Name things so they explain themselves
No abbreviations, and put units and qualifiers at the end so names sort and grep well. Comments explain why, not what. Keep variables in the smallest scope.
```rust
let latency_ms_max = 250;      // not `lat` or `maxLatencyMs`
let buffer_bytes_len = 4_096;
```

## 14. Do it right the first time
No "fix later" debt, and every change ships with its tests. Set important options explicitly instead of trusting defaults.
```rust
PgPoolOptions::new()
    .max_connections(16)                        // explicit, not the default
    .acquire_timeout(Duration::from_secs(2))
```

## Where to relax the rules
For web services and application code, "no allocation after startup" becomes "bounded allocation per request," and you can't control recursion inside dependencies, so cap input size and nesting instead. The rest still apply almost unchanged.

I can also produce a one-page printable cheat sheet, or the same summary for Java.
