# 16. User data storage architecture

Date: 2025-07-23

## Status

Accepted

## Context

WAD needs persistent storage for user-specific metadata (vacation days, absence records, future annotations) that complements Watson time tracking data. We evaluated JSON files vs SQLite for storing structured but extensible data with simple CRUD operations.

## Links

- Builds on [ADR 0014](0014-configuration-system-architecture.md) - Configuration system architecture

## Decision

We will implement **JSON file storage with trait composition** architecture:

- Module structure: `src/wad_data/` with feature-based organization (`absence.rs`, future `annotations.rs`)
- Trait composition: `WadDataStore: AbsenceStorage + ...` pattern for extensibility  
- JSON serialization using `serde_json` for human-readable, backup-friendly storage
- `HashMap<NaiveDate, AbsenceRecord>` return types for O(1) date lookups
- Storage location via `dirs-rs` in platform-appropriate data directory

**This choice aligns with Watson's native JSON storage format and prioritizes simplicity over database complexity.**

## Consequences

- Easy manual inspection and editing of stored data
- Simple backup and sync capabilities (plain files)
- Type-safe serialization with Rust's serde ecosystem
- Efficient date-based queries via HashMap structure
- Consistent data format with Watson's existing JSON storage
- Easy migration path to SQLite if complex queries become necessary