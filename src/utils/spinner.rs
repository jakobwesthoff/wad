use spinners::{Spinner, Spinners};
use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SpinnerConfig {
    message: String,
    debounce_ms: u64,
    spinner_type: Spinners,
}

impl Default for SpinnerConfig {
    fn default() -> Self {
        Self {
            message: "Collecting Data...".to_string(),
            debounce_ms: 300,
            spinner_type: Spinners::Dots,
        }
    }
}

impl SpinnerConfig {
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn with_debounce_ms(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }

    pub fn with_spinner_type(mut self, spinner_type: Spinners) -> Self {
        self.spinner_type = spinner_type;
        self
    }
}

pub struct SpinnerGuard {
    spinner: Arc<Mutex<Option<Spinner>>>,
    sender: Option<mpsc::Sender<SpinnerMessage>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

enum SpinnerMessage {
    Stop,
    StopWithMessage(String),
    StopWithSymbol(String, Option<String>),
}

impl SpinnerGuard {
    pub fn new(config: SpinnerConfig) -> Self {
        let (sender, receiver) = mpsc::channel();
        let spinner = Arc::new(Mutex::new(None));
        let spinner_clone = Arc::clone(&spinner);
        let debounce_duration = Duration::from_millis(config.debounce_ms);

        let thread_handle = thread::spawn(move || {
            let _start_time = Instant::now();

            // Wait for debounce delay or stop message
            match receiver.recv_timeout(debounce_duration) {
                Ok(message) => {
                    // Received stop message before debounce delay
                    Self::handle_message(message, &spinner_clone, false);
                    return;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Debounce delay elapsed, start spinner
                    let mut spinner_lock = spinner_clone.lock().unwrap();
                    *spinner_lock = Some(Spinner::new(config.spinner_type, config.message));
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // Channel disconnected, exit
                    return;
                }
            }

            // Listen for stop messages
            if let Ok(message) = receiver.recv() {
                Self::handle_message(message, &spinner_clone, true);
            }
        });

        Self {
            spinner,
            sender: Some(sender),
            thread_handle: Some(thread_handle),
        }
    }

    pub fn finish(mut self, message: Option<String>) {
        self.send_message(match message {
            Some(msg) => SpinnerMessage::StopWithMessage(msg),
            None => SpinnerMessage::Stop,
        });

        // Block until spinner thread fully completes
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn finish_with_symbol(mut self, symbol: String, message: Option<String>) {
        self.send_message(SpinnerMessage::StopWithSymbol(symbol, message));

        // Block until spinner thread fully completes
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    fn send_message(&mut self, message: SpinnerMessage) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(message);
        }
    }

    fn handle_message(
        message: SpinnerMessage,
        spinner: &Arc<Mutex<Option<Spinner>>>,
        spinner_active: bool,
    ) {
        let mut spinner_lock = spinner.lock().unwrap();

        if let Some(mut sp) = spinner_lock.take() {
            match message {
                SpinnerMessage::Stop => {
                    sp.stop(); // Stop the spinner thread
                    // Manually clear the line without newline
                    // The spinners crate doesn't provide a way to clear the line without
                    // adding a newline. All stop_* methods use writeln!() which adds \n.
                    // We want to clear the spinner but stay on the same line for our output.
                    eprint!("\x1b[2K\r");
                    io::stderr().flush().unwrap();
                }
                SpinnerMessage::StopWithMessage(msg) => {
                    sp.stop_with_message(msg);
                }
                SpinnerMessage::StopWithSymbol(symbol, msg) => {
                    sp.stop_with_symbol(&symbol);
                    if let Some(msg) = msg {
                        println!(" {}", msg);
                    }
                }
            }
        } else if !spinner_active {
            // Spinner never started, but we might want to show the completion message
            match message {
                SpinnerMessage::StopWithMessage(msg) => {
                    println!("{}", msg);
                }
                SpinnerMessage::StopWithSymbol(symbol, msg) => {
                    if let Some(msg) = msg {
                        println!("{} {}", symbol, msg);
                    }
                }
                SpinnerMessage::Stop => {
                    // Nothing to do
                }
            }
        }
    }
}

impl Drop for SpinnerGuard {
    fn drop(&mut self) {
        // Only send stop message and join if thread hasn't been taken by finish methods
        if self.thread_handle.is_some() {
            self.send_message(SpinnerMessage::Stop);

            if let Some(handle) = self.thread_handle.take() {
                let _ = handle.join();
            }
        }
    }
}
