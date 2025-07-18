# 12. Command default options strategy

Date: 2025-07-18

## Status

Accepted

## Context

Watson Dashboard needs to support interactive command selection when no command is specified. This requires all commands to work without mandatory arguments to enable seamless execution from the selection interface.

## Links

- Builds on [ADR 0003](0003-command-architecture-design.md) - Command architecture design
- Builds on [ADR 0004](0004-interactive-selection-library-choice.md) - Interactive selection library choice

## Decision

We will enforce a **command default options strategy** where every command must have sensible defaults and work without any arguments:

- Every command MUST be executable with zero arguments
- Commands MUST provide meaningful output using default settings
- Optional arguments MAY enhance behavior but are never mandatory
- Commands executed from interactive selection use default arguments only

**This is a design constraint we are enforcing for now to make interactive selection work effectively.**

## Consequences

- Commands can be discovered and executed immediately via interactive selection
- All commands follow consistent execution pattern
- Commands must be designed with meaningful defaults upfront
- Users might not discover optional enhancements without documentation
