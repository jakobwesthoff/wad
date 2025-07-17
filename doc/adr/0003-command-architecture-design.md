# 3. Command architecture design

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs a command architecture that supports multiple commands with consistent interfaces, automatic help generation, and easy extensibility. The application will support commands like `worktime`, `status`, `report`, and future commands.

Key requirements:
- Type-safe command definitions with compile-time validation
- Consistent command interface across all commands
- Integration with clap for argument parsing and help generation
- Support for command-specific arguments and options defined with clap derive
- Easy testing and mocking of individual commands
- Minimal boilerplate for adding new commands
- Static dispatch for optimal performance

## Decision

We will use a **enum_dispatch-based command architecture** that combines clap's derive API with static dispatch for optimal performance and minimal boilerplate.

**Command Trait:**
```rust
pub trait Command {
    fn run(&self) -> anyhow::Result<()>;
}
```

**Command Architecture:**
- Commands are defined as clap derive structs with their own argument parsing
- Each command implements the `Command` trait with a `run()` method
- Commands are registered in a single enum using `enum_dispatch` for automatic dispatch
- Clap handles argument parsing, help generation, and validation automatically
- Static dispatch eliminates runtime overhead while reducing boilerplate

**Integration Pattern:**
```rust
#[derive(Parser)]
#[enum_dispatch(Command)]
enum Commands {
    Worktime(WorktimeCommand),
    Status(StatusCommand),
}

#[derive(Parser)]
struct WorktimeCommand {
    #[arg(short, long)]
    weeks: u32,
}

impl Command for WorktimeCommand {
    fn run(&self) -> anyhow::Result<()> {
        // Implementation using self.weeks
    }
}
```

**Alternative approaches considered:**
- **Pure enum with manual match**: Requires touching multiple places for each new command
- **Inventory-based dynamic dispatch**: Runtime overhead and non-standard clap integration
- **Macro-generated registration**: Complex to implement and debug
- **Dynamic loading**: Runtime overhead and less type safety

## Consequences

**Positive:**
- **Type Safety**: Commands are validated at compile time with full clap derive support
- **Minimal Boilerplate**: Adding commands requires only enum registration and trait implementation
- **Standard Clap Integration**: Full support for clap derive, help generation, and argument validation
- **Static Dispatch**: Zero runtime overhead via enum_dispatch
- **Easy Testing**: Commands can be tested in isolation with their parsed arguments
- **Automatic Help**: Clap automatically generates help for each command and its arguments
- **Performance**: Optimal performance with compile-time dispatch
- **Maintainability**: Clear separation of concerns between commands

**Negative:**
- **Enum Registration**: New commands must be added to the Commands enum
- **Compile-time Registration**: Commands must be registered at compile time
- **Static Linking**: Cannot load commands dynamically at runtime
- **Dependency**: Requires enum_dispatch crate

**Risks:**
- **Command Conflicts**: Risk of name collisions between commands (mitigated by clap validation)
- **Forgotten Registration**: Risk of implementing command but forgetting to add to enum
- **Extensibility**: Third-party commands would require recompilation
