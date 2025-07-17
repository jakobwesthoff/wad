# 6. Watson CLI execution strategy

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs to execute Watson CLI commands to retrieve time tracking data. We need a consistent approach for command execution and output parsing.

## Decision

We will use **std::process::Command** to execute Watson CLI commands as subprocesses.

- Execute Watson CLI using `std::process::Command`
- Parse JSON output with `serde_json` where `--json` flag is supported
- Use `which` crate for cross-platform executable discovery
- Handle errors with structured `thiserror` types
- Provide `WatsonClient` abstraction around CLI interactions

## Consequences

**Positive:**
- Simple and straightforward implementation
- Works with any Watson CLI version
- Cross-platform compatibility
- Type-safe data structures

**Negative:**
- Subprocess overhead for each command
- Requires Watson CLI installation
- Vulnerable to Watson CLI output format changes
