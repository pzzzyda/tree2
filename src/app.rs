use std::path::Path;

use clap::Parser;

use crate::config::TreeConfig;
use crate::error::TreeError;
use crate::fmt::print_tree_with_config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct TreeArgs {
    path: String,

    #[arg(short, long, default_value_t = false)]
    all: bool,

    #[arg(short, long)]
    max_depth: Option<usize>,

    #[arg(short = 'd', long, default_value_t = false)]
    directories_only: bool,

    #[arg(long, default_value_t = false)]
    no_color: bool,

    #[arg(long, default_value_t = false)]
    ascii: bool,

    #[arg(long, default_value_t = false)]
    sort: bool,
}

pub fn run_tree_command(args: &TreeArgs) -> Result<(), TreeError> {
    let path = Path::new(args.path.as_str());

    let config = TreeConfig {
        max_depth: args.max_depth,
        show_hidden: args.all,
        dirs_only: args.directories_only,
        with_color: !args.no_color,
        ascii_only: args.ascii,
        sort: args.sort,
    };

    print_tree_with_config(path, &config)
}
