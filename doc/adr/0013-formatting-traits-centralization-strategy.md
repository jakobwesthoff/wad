# 13. Formatting traits centralization strategy

Date: 2025-07-18

## Status

Accepted

Supersedes [11. Duration formatting trait strategy](0011-duration-formatting-trait-strategy.md)

## Context

Watson Dashboard needs consistent formatting for various data types (durations, weeks, dates, etc.). With multiple formatting requirements emerging, we need a centralized strategy for trait-based formatting to maintain consistency and reusability.

## Links

- Links to [ADR 0008](0008-choice-of-ansi-color-library-for-cli-output.md) (Choice of ANSI color library)

## Decision

We will implement **formatting traits centralized in `utils::formatting`** with the following strategy:

- All formatting traits (e.g., `DurationFormat`, `WeekFormat`) reside in `utils::formatting`
- Each trait provides multiple format methods: `to_string_short()`, `to_string_long()`, etc.
- Traits follow consistent naming patterns: `to_string_<format_type>()`
- Implementation integrates with existing `owo-colors` semantic color system
- Each data type gets its own trait (no generic formatting interface)

## Consequences

- Centralized formatting logic in single module
- Consistent API across all data types
- Easy to maintain and extend formatting options
- Integrates with existing color and table infrastructure

