# 7. Watson CLI availability as hard requirement

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard has no meaningful functionality without Watson CLI. We need to decide when and how to check for Watson availability.

## Links

- Builds on [ADR 0006](0006-watson-cli-execution-strategy.md) - Watson CLI execution strategy

## Decision

We will make **Watson CLI availability a hard requirement** checked at application startup in `main()`.

- Check Watson CLI availability immediately in `main()` function
- Verify Watson CLI responds correctly to `--version` using `WatsonClient::is_usable()`
- Exit with clear error message if Watson CLI is not available
- Perform this check before any command execution

## Consequences

**Positive:**
- Fail fast with clear error messages
- All commands can assume Watson CLI is available
- Better user experience with actionable error messages

**Negative:**
- Minor startup time cost
- Cannot provide any functionality without Watson CLI
- Rigid installation dependency