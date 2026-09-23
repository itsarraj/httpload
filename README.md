# httpload

A simple, parallel cross-platform HTTP load generator. Fills the gap left by `wrk` (which isn't trivially cross-platform) or `ab` by providing a simple, native CLI HTTP benchmarking tool.

## Status

**built, untested**: the async request spawning and basic aggregation logic using Reqwest and Tokio is built. It has not been run against a real server to verify its throughput accuracy in this sandbox environment.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
httpload http://localhost:8080 --requests 1000 --concurrency 50
```
