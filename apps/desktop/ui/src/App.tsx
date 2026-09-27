// Phase 0 placeholder UI.
//
// No user-facing features are implemented in Phase 0. This component exists
// to prove that the Tauri shell, the frontend build pipeline, and the
// TypeScript/Tailwind toolchain are all wired together correctly.
//
// Real screens (Home, Workspaces, Sessions, Documents, Search, Tasks,
// Artifacts, Models, Tools, Help, Settings) begin in Phase 1.

export default function App() {
  return (
    <main className="flex min-h-screen items-center justify-center bg-neutral-950 text-neutral-100">
      <div className="text-center">
        <h1 className="text-3xl font-semibold tracking-tight">Machie</h1>
        <p className="mt-2 text-sm text-neutral-400">
          Phase 0 — Engineering Foundation
        </p>
      </div>
    </main>
  );
}