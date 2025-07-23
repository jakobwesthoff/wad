use crate::{
    commands::{
        config::ConfigCommand,
        worktime::{WorktimeTodayCommand, WorktimeWeeklyCommand},
    },
    config::Config,
    watson::WatsonClient,
};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;

pub mod config;
pub mod discovery;
pub mod worktime;

#[enum_dispatch]
pub trait Command {
    fn run(&self, watson_client: &WatsonClient, config: &Config, verbose: bool) -> Result<()>;
}

#[derive(Parser)]
#[enum_dispatch(Command)]
pub enum Commands {
    /// Configuration management
    #[command(name = "config")]
    Config(ConfigCommand),
    /// Show today's work time
    #[command(name = "worktime:today")]
    WorktimeToday(WorktimeTodayCommand),
    /// Show weekly work time overview
    #[command(name = "worktime:weekly")]
    WorktimeWeekly(WorktimeWeeklyCommand),
}
