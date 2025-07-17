use crate::watson::WatsonClient;
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

pub mod worktime;
pub use worktime::WorktimeTodayCommand;

#[enum_dispatch]
pub trait Command {
    fn run(&self, watson_client: &WatsonClient, verbose: bool) -> Result<()>;
}

#[derive(Parser)]
#[enum_dispatch(Command)]
pub enum Commands {
    /// Show today's work time
    #[command(name = "worktime:today")]
    WorktimeToday(WorktimeTodayCommand),
}
