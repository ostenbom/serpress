use clap::{Parser, Subcommand};
use thiserror::Error;

mod background_services;
mod check;
mod local_config;
mod start;

use start::start;

const SERPRESS_CONFIG_ENV: &str = "SERPRESS_CONFIG";
const SERPRESS_PORT: u16 = 9066;
const SERPRESS_STATE_FILE: &str = ".serpress-state";
const SERPRESS_PID_FILE: &str = ".serpress-pid";
const SERPRESS_CLOUDFLARED_PID: &str = ".serpress-cloudflared-pid";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("no valid state file: {0}")]
    NoState(String),
    #[error("no valid config provided: {0}")]
    BadConfig(String),
    #[error("could not save statefile: {0}")]
    SaveState(String),
    #[error("could not start local server: {0}")]
    StartLocalServer(String),
    #[error("could not start local tunnel: {0}")]
    StartLocalTunnel(String),
    #[error("could not load config to {0}: {1}")]
    LoadConfig(String, String),
    #[error("your session is in an inconsistent state. Stop your session before trying again.")]
    InconsistentState,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long)]
        config: Option<String>,
    },
    Stop {},
    Check {},
    Local {},
    Remote {},
}

fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    match &cli.command {
       Commands::Start{config}=> {
            start(config.clone())
       },
       Commands::Stop{} => {
        println!("Stop");
        Err(CliError::BadConfig(String::from("no good")))
       }
       Commands::Check{} => {
        println!("Check");
        Err(CliError::BadConfig(String::from("no good")))
       }
       Commands::Local{} =>{
         println!("Local");
        Err(CliError::BadConfig(String::from("no good")))
       }
       Commands::Remote{} => {
        println!("Remote");
        Err(CliError::BadConfig(String::from("no good")))
       }

    //    _Stop => println!("Stop"),
    //    _Check => println!("Check"),
    //    _Local => println!("Local"),
    }
}
