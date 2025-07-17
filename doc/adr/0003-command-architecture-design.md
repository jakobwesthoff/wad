# 3. Command architecture design

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs a command architecture that supports multiple commands with consistent interfaces, automatic help generation, and easy extensibility. The application will support commands like `help`, `worktime:weekly`, `worktime:today`, and future commands.

Key requirements:
- Type-safe command definitions with compile-time validation
- Consistent command interface across all commands
- Automatic help generation and command discovery
- Support for command-specific arguments and options
- Easy testing and mocking of individual commands
- Extensible architecture for adding new commands

## Decision

We will use a **trait-based command architecture** with compile-time command registration.

**Command Trait:**
```rust
pub trait Command {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self, args: &CommandArgs) -> anyhow::Result<()>;
}
```

**Command Registration:**
- Commands are registered at compile time using a registry pattern
- Each command implements the `Command` trait
- Commands are discovered automatically through static registration
- Help system iterates through registered commands

**Alternative approaches considered:**
- **Macro-based**: Complex to implement and debug
- **Dynamic loading**: Runtime overhead and less type safety
- **Clap subcommands**: Less flexible for complex command hierarchies

## Consequences

**Positive:**
- **Type Safety**: Commands are validated at compile time
- **Consistent Interface**: All commands follow the same pattern
- **Easy Testing**: Commands can be tested in isolation
- **Automatic Discovery**: Help system and command listing work automatically
- **Performance**: No runtime overhead for command lookup
- **Maintainability**: Clear separation of concerns between commands

**Negative:**
- **Boilerplate**: Each command requires trait implementation
- **Compile-time Registration**: Commands must be registered at compile time
- **Static Linking**: Cannot load commands dynamically at runtime

**Risks:**
- **Command Conflicts**: Risk of name collisions between commands
- **Registration Complexity**: Manual registration process could be error-prone
- **Extensibility**: Third-party commands would require recompilation
