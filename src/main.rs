mod app;
mod config;
mod error;
mod fmt;

use std::process::exit;

use clap::Parser;

use crate::app::TreeArgs;
use crate::app::run_tree_command;

fn main() {
    if let Err(e) = run_tree_command(&TreeArgs::parse()) {
        eprintln!("{}", e);
        exit(1);
    }
}
