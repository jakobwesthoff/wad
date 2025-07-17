# 2. Choice of table rendering library

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs a table rendering library for displaying time tracking data in formatted table structures. The application will display weekly worktime reports, daily summaries, and other tabular data that requires professional formatting.

Key requirements:
- Support for colored output and ANSI escape sequences for data visualization
- Flexible table formatting with headers, data rows, and spanning headers
- Good performance for datasets containing weeks/months of time tracking data
- Clean API for programmatic table construction from data structures
- Cross-platform compatibility across different terminal environments
- Support for dynamic column sizing and alignment

## Decision

We will use **tabled** (version 0.20+) as the table rendering library.

Alternative libraries considered:
- **comfy_table**: Good features but heavier weight and less performant
- **cli-table**: Limited formatting options and less actively maintained
- **prettytable-rs**: Older library with less modern API design

## Consequences

**Positive:**
- Better performance compared to comfy_table based on benchmarks
- Clean, modern API with derive macros for easy table construction
- Excellent formatting flexibility for complex table layouts
- Active maintenance and regular updates
- Good documentation and examples

**Negative:**
- Slightly more complex API than simpler alternatives
- Larger dependency tree than minimal table libraries
- Need to learn tabled-specific patterns and conventions

**Risks:**
- Performance characteristics under very large datasets need validation
- Color/ANSI handling needs testing across different terminals
