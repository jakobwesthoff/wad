# 5. Error handling strategy

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard requires a comprehensive error handling strategy that provides meaningful error messages to users while maintaining clean separation between library and application concerns. The application interacts with external systems (Watson CLI, file system) and processes user input, requiring robust error handling.

Key requirements:
- Structured error types that can be programmatically handled
- Rich error context for debugging and user feedback
- Clear separation between library errors and application errors
- User-friendly error messages without exposing internal details
- Proper error propagation through the application stack

## Decision

We will use a **hybrid error handling approach** combining `thiserror` for library-level errors and `anyhow` for application-level error handling.

**Library Layer (thiserror):**
- Each library module defines its own error type using `thiserror`
- Provides structured, typed errors with specific variants
- Enables proper error handling and recovery at module boundaries
- Examples: `WatsonClientError`, `TableRenderError`, `CommandLoaderError`

**Application Layer (anyhow):**
- Commands and main application logic use `anyhow::Result<T>`
- Convert library errors to `anyhow::Error` at command boundaries
- Add rich context using `.context()` for user-friendly messages
- Simplify error propagation with the `?` operator

## Consequences

**Positive:**
- **Structured Errors**: Library errors are typed and can be handled programmatically
- **Rich Context**: Application errors include context chains for debugging
- **User Experience**: Clear, actionable error messages for end users
- **Maintainability**: Clear separation between library and application concerns
- **Flexibility**: Library errors can be converted or wrapped as needed

**Negative:**
- **Complexity**: Two different error handling patterns to maintain
- **Conversion Overhead**: Need to convert between error types at boundaries
- **Learning Curve**: Developers need to understand both approaches

**Risks:**
- **Inconsistent Usage**: Risk of mixing patterns inappropriately
- **Context Loss**: Potential loss of structured error information when converting to anyhow
- **Performance**: Minor overhead from error conversion and context building
