use crate::watson::WatsonClient;
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

pub mod discovery;
pub mod worktime;
pub use worktime::{WorktimeTodayCommand, WorktimeWeeklyCommand};

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
    /// Show weekly work time overview
    #[command(name = "worktime:weekly")]
    WorktimeWeekly(WorktimeWeeklyCommand),
}
