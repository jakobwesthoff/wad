# 17. ULID adoption for user data record identification

Date: 2025-07-23

## Status

Accepted

## Context

WAD user data storage requires unique identifiers for individual records to enable precise targeting, modification, and removal operations. With multiple absence records possible per date, simple date-based addressing is insufficient for granular record management.

## Links

- Builds on [ADR 0016](0016-user-data-storage-architecture.md) - User data storage architecture

## Decision

We will use **ULID (Universally Unique Lexicographically Sortable Identifier)** for all user data records with the following approach:

- Every `AbsenceRecord` gets a unique ULID as primary identifier
- ULIDs provide chronological ordering by creation time (26-character base32)
- Storage operations use `(date, ULID)` for precise record targeting
- Records within each date file are sorted by ULID for consistent ordering
- Future data types (annotations, adjustments) will follow same ULID pattern

**This choice prioritizes time-sortable uniqueness over traditional UUID randomness.**

## Consequences

- Precise record identification enables selective operations on multi-record days
- Chronological ordering preserves creation sequence within each date
- Consistent 26-character identifiers across all user data types
- Efficient removal operations without full-file rebuilds
- Foundation for cross-referencing between different data types
- Slightly larger storage overhead compared to auto-incrementing integers