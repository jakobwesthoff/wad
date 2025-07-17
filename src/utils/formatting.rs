use owo_colors::{OwoColorize, colors::*};

// Semantic color type aliases
pub type SuccessColor = Green;
pub type ErrorColor = Red;
pub type WarningColor = Yellow;
pub type InfoColor = Cyan;
pub type VerboseColor = BrightMagenta;

/// Format success messages
pub fn success_text(text: &str) -> String {
    text.fg::<SuccessColor>().to_string()
}

/// Format error messages
pub fn error_text(text: &str) -> String {
    text.fg::<ErrorColor>().to_string()
}

/// Format warning messages
pub fn warning_text(text: &str) -> String {
    text.fg::<WarningColor>().to_string()
}

/// Format info messages
pub fn info_text(text: &str) -> String {
    text.fg::<InfoColor>().to_string()
}

/// Format headers/titles
pub fn header_text(text: &str) -> String {
    text.bold().to_string()
}

/// Format verbose/debug messages
pub fn verbose_text(text: &str) -> String {
    text.fg::<VerboseColor>().to_string()
}
