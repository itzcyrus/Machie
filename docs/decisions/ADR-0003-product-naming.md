# ADR-0003: Product Naming and Internal Architecture Distinction

Status: Accepted
Date: 2026-09-27

## Context

The product name must be settled before any user-visible strings, icons,
or documentation are written, so that no dual-naming scheme or pending
rename needs to be preserved.

Separately, the codebase has two conceptually distinct layers that are
easy to conflate if they are not named clearly:

- The reliable infrastructure — routing, task orchestration, security,
  permissions, providers, tools, runtime — which must never be bypassed.
- An optional, future, cosmetic/conversational layer built on top of the
  infrastructure.

If these two are treated as one thing, the personality layer will
gradually acquire the ability to bypass permissions, network policy, and
source grounding, because nothing in the code makes the boundary explicit.

## Decision

**The product name is Machie. This is final.** There is no pending rename
and no dual-naming scheme to preserve.

Internally, keep a clear architectural distinction even though there is
only one product name:

- **Machie Core** — the reliable infrastructure: routing, task
  orchestration, security, permissions, providers, tools, runtime. This is
  the part of the system that must never be bypassed.
- **Personality Layer** (also called **Companion Mode**) — an optional,
  future, cosmetic/conversational layer built *on top of* the Core. It is
  an internal architectural distinction, not a second product name, and it
  is not a separate product identity.

The Personality Layer, when it exists, may add personality, humor,
conversational style, voice, desktop presence, and contextual reactions,
but it must never bypass:

- permissions
- network policy
- security
- source grounding
- user confirmation
- tool validation

Personality never compromises correctness.

Product branding — name, icons, user-visible strings, copy — is
centralized in one place in the codebase. This is not because another
rename is anticipated (it is not), but because scattering brand strings
through the codebase is poor practice on its own merits.

## Rationale

- A single, settled name eliminates a whole class of confusion and
  rework.
- Making the Core / Personality Layer distinction architectural, not
  merely conceptual, is what keeps the personality layer honest: the
  boundary is expressed in code, not just in a document.
- Centralizing branding keeps the door open for future localization,
  theming, and product packaging without churn.

## Alternatives Considered

- **Treat Machie as a single undifferentiated entity in code:**
  rejected. The distinction between "infrastructure that must never be
  bypassed" and "optional layer on top" is load-bearing for the security
  model. Naming it now is cheap; retrofitting it later is not.
- **Adopt a dual naming scheme (one internal, one external):** rejected.
  Dual naming schemes are a maintenance tax with no compensating benefit.

## Consequences

- All user-facing strings reference "Machie," not a placeholder or a
  codename.
- The eventual Personality Layer is a distinct architectural layer that
  consumes the Core through its public interfaces; it does not live inside
  the Core.
- Reconsidering the product name requires a superseding ADR.