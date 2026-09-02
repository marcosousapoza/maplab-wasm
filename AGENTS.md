# Agent Boundaries

Work only in `maplab-wasm`. Generated declarations and the README define the
public browser contract. Never inspect or edit sibling repositories. File a
cross-repository issue when another component owns a required change.

Before completion, run rustfmt, Clippy with warnings denied, tests, and wasm-pack.
