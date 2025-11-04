use std::fs::DirEntry;
use std::path::Path;

use colored::Colorize;

use crate::config::TreeConfig;
use crate::config::TreeIndentType;
use crate::error::TreeError;

struct TreeTraversalState<'a> {
    indent_symbols: Vec<&'static str>,
    is_last_item: bool,
    current_depth: usize,
    config: &'a TreeConfig,
}

impl<'a> TreeTraversalState<'a> {
    pub fn new(config: &'a TreeConfig) -> Self {
        Self {
            indent_symbols: Vec::new(),
            is_last_item: true,
            current_depth: 1,
            config,
        }
    }

    pub fn prepare_parent_indent(&mut self) {
        if !self.indent_symbols.is_empty() {
            self.indent_symbols.pop();
            self.indent_symbols.push(if self.is_last_item {
                self.config.get_indent(TreeIndentType::Space)
            } else {
                self.config.get_indent(TreeIndentType::Vertical)
            });
        }
    }

    pub fn add_child_indent(&mut self, is_last_item: bool) {
        self.is_last_item = is_last_item;
        self.indent_symbols.push(if is_last_item {
            self.config.get_indent(TreeIndentType::Last)
        } else {
            self.config.get_indent(TreeIndentType::Continue)
        });
    }

    pub fn remove_indent(&mut self) {
        self.indent_symbols.pop();
    }

    pub fn print_current_indent(&self) {
        for i in &self.indent_symbols {
            print!("{}", i);
        }
    }
}

fn filter_entries(entries: Vec<DirEntry>, config: &TreeConfig) -> Result<Vec<DirEntry>, TreeError> {
    let mut filtered = Vec::new();

    for entry in entries {
        if !config.show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }

        if config.dirs_only
            && !entry
                .file_type()
                .map_err(|e| TreeError::FailedToGetFileType(entry.path().display().to_string(), e))?
                .is_dir()
        {
            continue;
        }

        filtered.push(entry);
    }

    Ok(filtered)
}

fn read_all_entries(path: &Path) -> Result<Vec<DirEntry>, TreeError> {
    let path_str = path.display().to_string();
    path.read_dir()
        .map_err(|e| TreeError::FailedToReadDir(path_str.clone(), e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TreeError::FailedToReadDirEntry(path_str, e))
}

fn print_path_name(path: &Path, state: &mut TreeTraversalState) -> Result<(), TreeError> {
    state.print_current_indent();

    let name = path.file_name().unwrap_or(path.as_os_str());
    if state.config.with_color {
        if path.is_dir() {
            println!("{}", name.to_string_lossy().blue().bold());
        } else if path.is_symlink() {
            println!("{}", name.to_string_lossy().cyan().bold());
        } else if path
            .metadata()
            .map_err(|e| TreeError::FailedToGetMetadata(path.display().to_string(), e))?
            .permissions()
            .readonly()
        {
            println!("{}", name.to_string_lossy().red().bold());
        } else {
            println!("{}", name.to_string_lossy());
        }
    } else {
        println!("{}", name.display());
    }

    Ok(())
}

fn print_tree_recursive(path: &Path, state: &mut TreeTraversalState) -> Result<(), TreeError> {
    if let Some(max_depth) = state.config.max_depth {
        if state.current_depth > max_depth {
            return Ok(());
        }
    }

    print_path_name(path, state)?;

    if !path.is_dir() {
        return Ok(());
    }

    state.current_depth += 1;

    state.prepare_parent_indent();

    let entries = read_all_entries(path)?;

    let mut filtered = filter_entries(entries, state.config)?;

    if state.config.sort {
        filtered.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    for (index, entry) in filtered.iter().enumerate() {
        state.add_child_indent(index == filtered.len() - 1);
        print_tree_recursive(entry.path().as_path(), state)?;
        state.remove_indent();
    }

    state.current_depth -= 1;

    Ok(())
}

pub fn print_tree_with_config(path: &Path, config: &TreeConfig) -> Result<(), TreeError> {
    print_tree_recursive(path, &mut TreeTraversalState::new(config))
}
