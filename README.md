# PoC of async deadlock

Modify the number of `worker_threads` and the value `spawned_tasks` to replicate the threads deadlock. If `worker_threads` <= `spawned_tasks` the main Tokio thread will execute the `generate()` trait and park itself, causing the runtime deadlock.


## Tokio tracing execution

```sh
$ RUSTFLAGS="--cfg tokio_unstable" RUST_LOG=trace,tokio=trace cargo run
```
