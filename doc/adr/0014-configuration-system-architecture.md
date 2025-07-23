# 14. Configuration system architecture

Date: 2025-07-23

## Status

Accepted

## Context

Watson Dashboard needs persistent configuration storage for user preferences to enable enhanced calculations and features beyond basic Watson CLI output. This requires cross-platform configuration file handling and integration with the existing command architecture.

## Links

- Builds on [ADR 0012](0012-command-default-options-strategy.md) - Command default options strategy

## Decision

We will implement a **layered configuration system** with the following architecture:

- Use `config` crate for robust configuration handling with multiple source support (files, environment variables)
- Use `dirs` crate for cross-platform config directory resolution 
- Add `Config` parameter to `Command` trait signature
- Implement dedicated `config` command for configuration management
- Store configuration as TOML in platform-appropriate config directory

Example: `WAD_WORKHOURS_PER_WEEK=35 wad worktime:weekly` overrides file-based config.

## Consequences

- Configuration persists across runs with sensible defaults
- Environment variable override capability for all config values
- All commands gain access to user configuration
- Foundation for advanced features like vacation tracking and plus hours calculation
- Commands must be updated to accept configuration parameter
