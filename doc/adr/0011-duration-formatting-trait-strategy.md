# 11. Duration formatting trait strategy

Date: 2025-07-17

## Status

Accepted

Superseded by [13. Formatting traits centralization strategy](0013-formatting-traits-centralization-strategy.md)

## Context

Watson Dashboard needs multiple duration formats across the application: eg. short format (hh:mm) and long format (X hours and Y minutes). We need a consistent, reusable approach.

## Decision

We will implement a **`DurationFormat` trait** with standard formatting methods like:

- `to_string_hhmm()` - returns "08:05" format
- `to_string_long_hhmm()` - returns "8 hours and 5 minutes" format
- The trait will be added to as needed
- Implement trait for `chrono::Duration` type


## Consequences

- Consistent duration formatting across application
- Extensible for future format requirements
- Reusable trait can be implemented on other types
