# 15. Test framework choice for data-driven tests

Date: 2025-07-23

## Status

Accepted

## Context

WAD data structures require comprehensive serialization testing with multiple test cases. We evaluated `test-case` vs `rstest` for parameterized testing to validate JSON serialization of `AbsenceRecord` and `AbsenceType` variants across different scenarios.

## Decision

We will use **`test-case`** for data-driven tests with the following rationale:

- Simple parameterized testing with `#[test_case]` attribute syntax
- Lightweight dependency footprint compared to `rstest`
- Sufficient for our serialization validation needs
- Clean syntax for straightforward parameter passing: `#[test_case(input, expected; "description")]`
- No need for complex fixtures or matrix testing features

**This choice prioritizes simplicity over advanced features we don't currently need.**

## Consequences

- Parameterized tests for JSON serialization validation
- Lower dependency overhead in test builds
- Limited to basic parameter passing (no fixtures or advanced features)
- Easy migration to `rstest` later if complex test scenarios emerge