# Fleet Status

A **real-time fleet monitoring CLI** that aggregates health checks across all SuperInstance deployed services — Cloudflare Workers, npm packages, and the vector search API — into a single console dashboard with latency measurement, uptime status, and conservation-law display.

## Why It Matters

When a fleet spans Cloudflare Workers, npm registries, and vector search endpoints, checking each service manually is impractical. Fleet Status centralizes the monitoring loop: it pings every HTTP endpoint, measures response latency, pulls vector index statistics (vector count, embedding dimensions), and verifies npm package publication status. The `watch` subcommand enables continuous monitoring with 30-second polling, making it suitable for NOC displays or incident response. Without this tool, operators would need 5+ browser tabs or curl scripts to verify fleet health.

## How It Works

**Endpoint probing** uses `reqwest` with a 10-second timeout per request. Each `check_endpoint` call measures wall-clock latency from the instant before the HTTP GET to the moment the response body is received. The result includes HTTP status code, latency in milliseconds, and truncated error messages for failed requests.

**Service registry** is a compile-time static array `HTTP_SERVICES` containing `{name, url}` pairs for each fleet endpoint. This avoids runtime configuration while keeping the service list version-controlled.

**API stats** query the fleet-vector-api `/stats` endpoint, which returns vector count and embedding dimensions from the Cloudflare Vectorize index. A sample search (`POST /search`) validates the full retrieval pipeline end-to-end.

**npm checks** query the npm registry (`registry.npmjs.org/{package}`) and extract the `dist-tags.latest` field to confirm packages are published and current.

**Conservation display** shows the SuperInstance physics invariant **γ + η = C** with current drift, providing a visual sanity check that the fleet's resource allocation hasn't drifted from equilibrium.

```
HTTP Service Check Loop:
  for each service in HTTP_SERVICES:
    GET service.url (timeout=10s)
    measure latency
    classify as ok/err based on status.is_success()

npm Check Loop:
  for each package in NPM_PACKAGES:
    GET registry.npmjs.org/{package}
    parse dist-tags.latest
```

Latency measurement precision: millisecond granularity via `Instant::elapsed()`. The reqwest client is reused across checks with connection pooling enabled.

## Quick Start

```bash
cargo run --                   # Overview of all services
cargo run -- check fleet-vector-api   # Detailed check of one service
cargo run -- api               # Vector API stats + sample search
cargo run -- crates            # List published SuperInstance crates
cargo run -- watch             # Continuous monitoring (30s poll)
```

## API

| Module | Function/Type | Description |
|--------|--------------|-------------|
| `api` | `get_stats(client)` | Fetch vector index stats |
| `api` | `search(client, query, top_k)` | Execute a vector search |
| `check` | `check_endpoint(client, name, url)` | Probe an HTTP endpoint |
| `check` | `HealthResult` | Result struct (status, latency, ok, error) |
| `check` | `build_client()` | Construct pooled reqwest client |
| `npm` | `get_package_version(client, name)` | Fetch npm latest version |
| `display` | `overview()` | Print full fleet status dashboard |
| `display` | `watch_loop()` | Continuous 30s polling mode |

## Architecture Notes

Fleet Status is the observability layer of the SuperInstance fleet. It exercises the same endpoints that production traffic uses, providing synthetic monitoring. The conservation law display (**γ + η = C**) connects fleet health to the physics model: when γ (coordination overhead) is low and η (reflexive automation) is high, the fleet is in equilibrium. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- reqwest HTTP client: https://docs.rs/reqwest
- Cloudflare Workers health endpoints: https://developers.cloudflare.com/workers/observability/
- Tokio async runtime: Tokio Project, "Asynchronous Programming in Rust" (2024).

## License

MIT
