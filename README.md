# Retry policy reference (Rust)

Private reference repository for a live `oc-port-agent` integration test. `retry_delay_ms` implements capped exponential backoff for unsigned 32-bit attempt indices; the CLI prints JSON results for differential comparison.

```sh
cargo test
cargo run --quiet -- 0 1 2 3 4 32
npm ci
npm run port -- --help
```

The tool is installed from `tools/oc-port-agent.tgz`. This is the source repo; no reverse synchronization is enabled. The downstream is [oc-port-demo-typescript](https://github.com/kyeshmz/oc-port-demo-typescript).
