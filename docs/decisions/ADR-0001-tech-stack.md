# ADR-0001: Technology Stack

Status: Accepted
Date: 2026-09-27

## Context

Machie is a local-first intelligent workspace that will eventually have
permission to touch the user's filesystem, processes, and network. The
application must not only be performant but also safe by construction:
model lifecycle state, concurrent file access, and permission enforcement
are the kinds of problems where whole classes of bugs must be prevented,
not merely mitigated by developer discipline.

The performance-critical work in Machie — model inference, OCR, media
transcoding, PDF parsing — already happens in optimized native libraries
regardless of the orchestrator language. The language choice therefore
does not primarily determine raw speed; it determines what kinds of
mistakes are structurally possible.

## Decision

The following stack is adopted, effective immediately, and is not open for
re-litigation without a superseding ADR:

- **Core / orchestration language:** Rust (Cargo workspace, `crates/*`).
- **Desktop shell:** Tauri 2.
- **UI framework:** React + TypeScript + Tailwind CSS.
- **Database:** SQLite, accessed only from Rust.
- **Full-text search:** SQLite FTS5.
- **Vector index:** Local; to be evaluated in Phase 3, not decided here.
  Not to be implemented early.
- **Local inference:** llama.cpp, via FFI or a supervised subprocess from
  Rust.
- **Config format:** TOML, versioned.
- **Frontend package manager:** pnpm.

Additional standing constraints:

- Rust is the only language for core logic.
- Backend logic is never built in the frontend.
- The UI is a presentation layer only. It renders state and issues typed
  commands to the Rust core; it never makes routing, permission, or
  provider-selection decisions.
- A second language for core logic, a substitute database, a different
  gateway shell, or a different build system requires a superseding ADR.
- C/C++ libraries (llama.cpp, FFmpeg, Tesseract) remain fully usable via
  FFI; they are not reimplemented.

## Rationale

- **Rust over C:** memory safety. This system will eventually have
  permission to modify user files and control user processes. Making
  memory safety the developer's manual responsibility is not acceptable
  here. Rust's ownership model makes data races on model lifecycle state
  and unsafe concurrent filesystem access structurally impossible, not
  merely discouraged.
- **Rust over C++:** same reasoning for the primary language. C and C++
  libraries remain usable via FFI, so nothing is lost.
- **Tauri over a heavier shell:** Tauri 2 keeps the desktop shell thin and
  provides a clean, typed Rust↔UI boundary.
- **SQLite over a server database:** local-first, single-user desktop
  application. A server database would introduce an operational
  dependency that provides no benefit at this stage.
- **TOML over JSON/YAML for config:** human-editable, unambiguous, and
  trivially versionable.
- **pnpm over npm/yarn:** faster, disk-efficient, and content-addressed.
- **React + TypeScript + Tailwind:** mature ecosystem, first-class typing,
  and a styling system that scales without bespoke CSS architecture.

## Alternatives Considered

- **Python (as core language):** rejected. Would make performance-critical
  paths dependent on native extensions anyway, while adding a dynamic
  runtime with weaker compile-time guarantees for permission and
  lifecycle invariants.
- **Go:** rejected. Memory-safe, but a GC-based runtime introduces pause
  and memory-accounting behaviors that complicate resource-aware model
  scheduling, and the FFI boundary to llama.cpp is less ergonomic than
  Rust's.
- **C++ as primary language:** rejected. Unsafe by default in exactly the
  areas (lifecycle state, concurrent file access) where Machie cannot
  afford developer error.
- **Electron instead of Tauri:** rejected. Much larger runtime footprint
  and a weaker native boundary for a local-first desktop application.
- **PostgreSQL instead of SQLite:** rejected. Adds an operational
  dependency that has no benefit for a single-user desktop application.

## Consequences

- The core has no dependency on the UI framework; replacing Tauri/React
  with a native GUI (GTK4/Libadwaita, Slint, egui) later is a
  presentation-layer replacement, not a core rewrite.
- Every provider and tool boundary must be expressible in Rust traits.
- The UI cannot "reach around" the core; all decisions flow through the
  core, which is the point.
- Reconsidering this stack requires a superseding ADR.