# 4. Interactive selection library choice

Date: 2025-07-17

## Status

Accepted

## Context

Watson Dashboard needs an interactive selection mechanism for choosing commands when no specific command is provided. This provides a user-friendly interface for command discovery and execution without requiring users to memorize all available commands.

Key requirements:
- Fuzzy search capabilities for command filtering
- Clean, intuitive user interface
- Cross-platform terminal compatibility
- Keyboard navigation support
- Integration with existing command system
- Minimal performance overhead
- Consistent behavior across different terminal environments

## Decision

We will use **inquire** (version 0.7+) as the interactive selection library.

**Rationale:**
- Lightweight and focused on core selection functionality
- Built on crossterm for cross-platform compatibility
- Clean API that integrates well with our command architecture
- Good performance characteristics
- Active maintenance and development

**Alternative libraries considered:**
- **dialoguer**: More features but heavier weight, includes unnecessary UI components
- **External fzf**: Excellent fuzzy search but requires external dependency
- **Custom implementation**: Would require significant development effort

## Consequences

**Positive:**
- **Lightweight**: Minimal impact on binary size and startup time
- **Cross-platform**: Works consistently across different operating systems
- **Integration**: Clean API that fits well with our command system
- **User Experience**: Intuitive interface for command discovery
- **Maintenance**: Well-maintained library with regular updates

**Negative:**
- **Dependency**: Additional dependency in the project
- **Feature Limitations**: Less feature-rich than alternatives like dialoguer
- **Customization**: Limited customization options for advanced UI needs

**Risks:**
- **Terminal Compatibility**: Potential issues with exotic terminal configurations
- **Performance**: UI responsiveness with large numbers of commands
- **Maintenance**: Risk of library abandonment or breaking changes
