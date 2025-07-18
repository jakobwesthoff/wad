# 8. Choice of ANSI color library for CLI output

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs colored CLI output for better user experience. We need a library that provides elegant API, good performance, and aligns with Rust best practices.

## Decision

We will use **owo-colors** for ANSI colored CLI output.

**Alternatives considered:**
- **colored** - Simple API but allocates strings
- **console** - Feature-rich but heavyweight with higher overhead
- **anstyle** - Foundation layer, not end-user friendly

**Why owo-colors:**
- Zero-allocation design with zero-cost abstractions
- Most Rust-idiomatic API using extension traits
- no_std compatible and actively maintained
- Drop-in replacement for colored if needed

## Links

- Links to [ADR 0013](0013-formatting-traits-centralization-strategy.md) (Formatting traits centralization strategy)

## Consequences

**Positive:**
- Elegant extension trait API: `"text".green().bold()`
- Zero runtime overhead with compile-time optimization
- Consistent with project's performance and type safety focus

**Negative:**
- Slightly newer library with smaller ecosystem
- Less documentation compared to colored
