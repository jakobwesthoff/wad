use std::fmt::Display;

use inquire::Select;

/// A wrapper that pairs a display string with any value type for selection menus
#[derive(Debug, Clone)]
pub struct SelectionOption<T> {
    display: String,
    value: T,
}

impl<T> SelectionOption<T> {
    pub fn new(display: impl Into<String>, value: T) -> Self {
        Self {
            display: display.into(),
            value,
        }
    }

    pub fn value(self) -> T {
        self.value
    }
}

impl<T> Display for SelectionOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// A helper for creating selection menus with typed values
pub struct SelectionMenu<T> {
    prompt: String,
    options: Vec<SelectionOption<T>>,
}

impl<T> SelectionMenu<T> {
    /// Create from items that already implement Display
    pub fn from_display_items<I>(prompt: impl Into<String>, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Display + Clone,
    {
        let options = iter
            .into_iter()
            .map(|item| SelectionOption::new(item.to_string(), item))
            .collect();

        Self {
            prompt: prompt.into(),
            options,
        }
    }

    pub fn prompt(self) -> Result<T, inquire::InquireError> {
        let selection = Select::new(&self.prompt, self.options).prompt()?;
        Ok(selection.value)
    }
}
