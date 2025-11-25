use std::path::Path;

use clap::Parser;

use crate::config::TreeConfig;
use crate::error::TreeError;
use crate::fmt::print_tree_with_config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct TreeArgs {
    #[arg(required = true, help = "Path to the directory to display")]
    path: String,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Show hidden files and directories"
    )]
    all: bool,

    #[arg(long, help = "Limit the depth of the directory tree to display")]
    level: Option<usize>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Show only directories, not files"
    )]
    directories_only: bool,

    #[arg(long, default_value_t = false, help = "Disable colorized output")]
    no_color: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Use ASCII characters only for tree display"
    )]
    ascii: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Disable sorting of directory entries"
    )]
    no_sorting: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Show hidden files and directories (ignores .gitignore)"
    )]
    show_gitignore: bool,
}

pub fn run_tree_command(args: &TreeArgs) -> Result<(), TreeError> {
    let path = Path::new(args.path.as_str());

    let config = TreeConfig {
        max_depth: args.level,
        show_hidden: args.all,
        dirs_only: args.directories_only,
        with_color: !args.no_color,
        ascii_only: args.ascii,
        sort: !args.no_sorting,
        ignore_gitignore: !args.show_gitignore,
    };

    print_tree_with_config(path, &config)
}
