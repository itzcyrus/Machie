# Router golden test sets

The router (spec Part Q.6) is the most consequential component in Machie.
A misclassification silently breaks the deterministic-tools philosophy.
Every router change must run against these sets before merging.

## Format

One JSON object per line. Blank lines and lines beginning with `#` are
ignored.

```json
{"input": "Convert this PDF to Markdown", "expected_type": "ACTION", "expected_tool": "document.convert"}
{"input": "What does this report say about Q3 revenue?", "expected_type": "ANSWER"}
{"input": "Find my meeting notes from last week", "expected_type": "SEARCH"}
```
    input (string, required) — raw user request.

    expected_type (string, required) — one of ANSWER, SEARCH, ACTION,
    TRANSFORM, RESEARCH, COMPARE, GENERATE, INDEX, EXECUTE.

    expected_tool (string, optional) — the tool the router should select,
    when the case is specifically a tool-selection test.

Files

    answer.jsonl — ANSWER intent cases

    search.jsonl — SEARCH intent cases

    action.jsonl — ACTION intent cases (including deterministic tool selection)

    ambiguous.jsonl — cases where the correct behavior is to ask, defer, or
    fall back predictably

    security.jsonl — prompt injection, privilege escalation attempts, network
    policy bypass attempts. These are the security test surface for the router.

    regression.jsonl — previously broken cases that must stay fixed

Harness
```
cargo run --bin router-eval
```


