
# Examples of unsafe async Rust

I put together this small collection of simple examples that can easily showcase the some of the async footguns that the compiler / clippy / etc wont warn you about, but can have some serious consequences (deadlocks, or panics).

There are three main categories I've seen in the wild:
 * mixing blocking behavior inside async contexts.
 * using std mutex with async.
    * I categorize this as its own subtopic, since it doesn't just cause weird scheduling clogs -- it can cause full on deadlocks.
 * cancellation unsafe behaviors.

These currently all just have one simple example of working vs broken code so they can be easily ran, looked at, and tweaked to get a feel for how exactly these footguns can catch you.

If you're interested, running the examples is as simple as cloning the repo and running:
```
$ cargo run --example cancellation-2-unsafe
```
