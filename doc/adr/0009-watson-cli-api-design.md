# 9. Watson CLI API Design

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs a consistent API design for interacting with Watson CLI commands. We need clear method naming conventions and structured parameter handling.

## Links

- Builds on [ADR 0006](0006-watson-cli-execution-strategy.md) - Watson CLI execution strategy

## Decision

We will use **semantic method naming** and **structured query objects** for Watson CLI interactions:

- Method naming: `watson_client.log(query)` not `execute_log_command`
- Use `LogQuery` data structures for command parameters instead of raw CLI arguments
- Return structured data types (`Frames`) from CLI interactions

## Consequences

- Clear, semantic API that matches Watson CLI commands
- Type-safe parameter handling with LogQuery
- Consistent interface for future Watson commands
