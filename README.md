# fleet-status

CLI tool for monitoring all SuperInstance fleet services.

## Install

```bash
cargo install --path .
```

## Usage

```bash
$ fleet-status
SuperInstance Fleet Status
==========================
✅ fleet-vector-api       .... 200 OK (45ms)
   vectors: 1012, dimensions: 384
✅ fleet-edge              .... 200 OK (32ms)
✅ superinstance-assets    .... 200 OK (28ms)
✅ superinstance.ai        .... 200 OK (120ms)
✅ @superinstance/tminus-client      .... v1.0.0 (npm)
✅ @superinstance/tminus-dispatcher  .... v1.0.0 (npm)

Conservation: γ=0.72 η=0.28 C=1.00 drift=0.00

Bottles in transit: 0
```

### Subcommands

| Command | Description |
|---------|-------------|
| `fleet-status` | Overview of all services (default) |
| `fleet-status check [service]` | Detailed check of one service |
| `fleet-status api` | Show API stats, vector count, sample search |
| `fleet-status crates` | List published crates.io packages |
| `fleet-status watch` | Continuous monitoring (polls every 30s) |

## Services Monitored

- **fleet-vector-api** — Vector search API (BGE-small-en-v1.5, 384-dim)
- **fleet-edge** — Edge worker
- **superinstance-assets** — OG images, favicon, badge generation
- **superinstance.ai** — GitHub Pages homepage
- **npm packages** — @superinstance/tminus-client, @superinstance/tminus-dispatcher
- **crates.io** — 38+ Rust crates under superinstance
