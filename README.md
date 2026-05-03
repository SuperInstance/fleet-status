# Fleet Status

> The lighthouse radar — seeing every vessel in the fleet, always.

**Topics:** `fleet-monitoring` `agent-registry` `lighthouse` `keeper` `live-status` `cocapn`

Live snapshot of the [Cocapn fleet](https://github.com/SuperInstance) — agent registry, published packages, live services, and fleet architecture.

---

## What It Does

Fleet Status is the **lighthouse radar** — a live view of every agent, service, and package in the Cocapn fleet. It aggregates data from the keeper service so you can see at a glance who's up, what's deployed, and how the fleet is doing.

**Topics:** `fleet-monitoring` `agent-registry` `lighthouse` `keeper` `live-status` `cocapn`

## Quick Start

```bash
# Check live fleet status
curl http://localhost:8900/fleet/status

# List all registered agents
curl http://localhost:8900/agents

# Check specific agent health
curl http://localhost:8900/agents/oracle1/health

# Get fleet-wide metrics
curl http://localhost:8900/fleet/metrics
```

## Architecture

```
fleet-status/
├── README.md               # This file
├── FLEET-CONTEXT-TILE.md   # Fleet context tile
└── LICENSE
```

The status data comes from the **keeper service** running at `keeper:8900`. The keeper is the fleet's central registry — every agent checks in, every service announces itself, every package publishes its version on deploy.

## Live Data

The keeper serves JSON endpoints:

| Endpoint | What You Get |
|----------|-------------|
| `GET /fleet/status` | Full fleet snapshot |
| `GET /fleet/metrics` | Tile counts, room counts, agent cycle counts |
| `GET /agents` | All registered agents |
| `GET /agents/{name}/health` | Single agent health |
| `GET /services` | All live services |
| `GET /packages` | Published packages (PyPI, crates.io) |

## Example Response

```bash
curl http://localhost:8900/fleet/status
```

```json
{
  "timestamp": "2026-05-03T12:00:00Z",
  "agents": [
    {"name": "Oracle1", "status": "ACTIVE", "cycle": 42},
    {"name": "JetsonClaw1", "status": "ACTIVE", "cycle": 15}
  ],
  "services": {
    "keeper": "UP",
    "plato_server": "UP",
    "seed_mcp": "DOWN"
  },
  "packages": {
    "pypi": 25,
    "crates": 14
  }
}
```

## Fleet Context

Part of the Cocapn fleet. Related repos:
- [fleet-orchestrator](https://github.com/SuperInstance/fleet-orchestrator) — Stateless edge coordination hub
- [deckboss](https://github.com/SuperInstance/deckboss) — Agent Edge OS for persistent backends
- [keeper](https://github.com/SuperInstance/keeper) — Fleet registry and discovery service
- [lighthouse-monitor](https://github.com/SuperInstance/lighthouse-monitor) — Service monitoring

---
🦐 Cocapn fleet — lighthouse keeper architecture
