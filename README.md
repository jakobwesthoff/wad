# Watson Dashboard

A CLI-based dashboard for the Watson time tracker.

## About

The Watson time tracker is a great tool for tracking time on the command line. However, I was missing certain overviews and functionality for analyzing tracked data. This dashboard provides tools to query Watson data in ways that make it more useful for my workflow.

Since this functionality might be useful to others, I'm publishing it as open source. This project is intended less as a fully-featured tool and more as a starting point for ideas or building blocks to solve your personal Watson requirements.

The name "Dashboard" might be a bit much - it mainly provides simple tooling to access Watson data in a way that makes usage easier for me. It will grow over time as I implement things I currently do by hand, particularly when they become repetitive and annoying enough to motivate me to actually implement a solution.

## Example Usage

```bash
# Show today's total work time
wad worktime:today

# Show breakdown by projects  
wad worktime:today --projects

# Show what commands and options exist
wad --help

# Show the specific options of a specific commands
wad worktime:today --help
```

## Requirements

- Watson CLI must be installed and accessible in your PATH

## Development

Written in Rust. Requires [Rust/Cargo](https://rustup.rs/) to build.

```bash
cargo build --release
```
