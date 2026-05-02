# Fleet Status

Live snapshot of the Cocapn fleet — agent registry, published packages, live services, and fleet architecture.

## Brand Line
> The lighthouse radar — seeing every vessel in the fleet, always.

## Usage

Fleet status is generated from the live keeper service at `keeper:8900`.

```bash
# Check live fleet status
curl http://localhost:8900/fleet/status

# List all registered agents
curl http://localhost:8900/agents

# Check specific agent health
curl http://localhost:8900/agents/oracle1/health
```

## Fleet Context

Part of the Cocapn fleet. Related repos:
- [fleet-orchestrator](https://github.com/SuperInstance/fleet-orchestrator) — Stateless edge coordination hub
- [deckboss](https://github.com/SuperInstance/deckboss) — Agent Edge OS for persistent backends
- [keeper](https://github.com/SuperInstance/keeper) — Fleet registry and discovery service

---
🦐 Cocapn fleet — lighthouse keeper architecture