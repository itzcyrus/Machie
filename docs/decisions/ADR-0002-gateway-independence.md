# ADR-0002: Gateway Independence

Status: Accepted
Date: 2026-09-27

## Context

Machie will eventually talk to inference gateways (OmniRoute, OpenRouter,
and possibly direct provider APIs). The temptation, when building the
first gateway integration, is to let the gateway's shape leak into the
core: to let the router reason about gateway-specific capabilities, to
special-case "if the user has OmniRoute configured," or to make the core
assume a gateway is present.

That path leads directly to a product that is a client for one gateway,
which is explicitly not what Machie is.

## Decision

Gateway independence is mandatory. The core depends on a `GatewayProvider`
trait, never on any specific gateway:
```
Machie Core → GatewayProvider trait → { OmniRoute, OpenRouter, Direct APIs }
```

**Not:**
```
Machie Core → OmniRoute → everything
```

Concretely:

1. The core must remain fully functional (local models, local tools, local
   search, documents, workspaces) with **zero** gateways configured or
   reachable.
2. No file outside a gateway's own provider module may reference
   `omniroute` or `openrouter` by name.
3. Core code asks `provider.supports(Capability::...)`, never
   `if provider_name == "..."`.
4. Provider-specific behavior lives inside that provider's own module.
5. Adding a new gateway is a matter of implementing the trait, not
   modifying the router.
6. If a configured gateway becomes unreachable or is removed, its provider
   is marked unavailable and the router falls back per the user's fallback
   policy or to local execution — never silently, and never treated as an
   application-level crisis.
7. Gateway-specific configuration (e.g. OmniRoute's default local endpoint)
   is configurable, never hard-coded as the only possible value. Remote
   self-hosted instances are first-class and are clearly distinguished
   from local ones under the network policy.

## Rationale

- The abstraction is the product. Machie is a router and orchestrator; if
  it collapses into a client for whatever gateway was integrated first,
  its core value proposition is lost.
- The core's local-first guarantees (LOCAL_ONLY network mode, no silent
  cloud fallback, no silent uploads) are only defensible if no gateway is
  privileged enough to bypass them.
- Swapping gateways, or running with none, must be a configuration change,
  not a code change.
- Third-party gateways have their own maintenance and API-stability
  trajectories. Coupling the core to one is a liability, not a feature.

## Alternatives Considered

- **First-class support for one gateway, "others later":** rejected. This
  is the path to the core assuming the gateway's shape, after which adding
  a second gateway requires rewriting the router. The whole point of the
  trait is to make the second gateway trivial.
- **A generic HTTP adapter instead of a trait:** rejected. Provider
  capabilities (tool calling, structured output, streaming, vision,
  reasoning, context length) differ in ways a generic HTTP layer cannot
  express. The trait is the right granularity.
- **Let the router reason about gateway identity:** rejected. That is
  precisely the leak this ADR exists to prevent.

## Consequences

- Every provider, including the local llama.cpp provider, implements the
  same trait family (spec Part C.1).
- Before implementing any specific gateway integration, its current API
  stability and maintenance status must be verified. No speculative
  behavior is built around unverified third-party projects.
- Provider selection is a capability-matching problem, not an
  identity-matching problem.
- Reconsidering this rule requires a superseding ADR.