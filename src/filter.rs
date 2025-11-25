use std::fs::DirEntry;
use std::fs::read_to_string;
use std::path::Path;

use crate::config::TreeConfig;
use crate::error::TreeError;

pub struct GitignoreFilter {
    rules: Vec<String>,
}

impl GitignoreFilter {
    pub fn build_from_gitignore(gitignore_path: &Path) -> Result<Option<Self>, TreeError> {
        if !gitignore_path.exists() {
            return Ok(None);
        }

        let content = read_to_string(gitignore_path)
            .map_err(|e| TreeError::ReadGitignore(gitignore_path.display().to_string(), e))?;

        let rules = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.to_string())
            .collect();

        Ok(Some(Self { rules }))
    }

    pub fn should_ignore(&self, path: &Path, is_dir: bool) -> bool {
        if let Some(file_name_os) = path.file_name() {
            let file_name = file_name_os.to_string_lossy();

            for rule in &self.rules {
                if self.matches_rule(&file_name, rule, is_dir) {
                    return true;
                }
            }
        }

        false
    }

    fn matches_rule(&self, path: &str, rule: &str, is_dir: bool) -> bool {
        // Simplified .gitignore rule matching implementation.

        // Handle rules that start with / (only match files in the root directory)
        let rule_pattern = if rule.starts_with('/') {
            &rule[1..]
        } else {
            rule
        };

        // Handle rules that end with / (only match directories)
        let dir_only = rule_pattern.ends_with('/');
        if dir_only && !is_dir {
            return false;
        }
        let rule_pattern = if dir_only {
            &rule_pattern[..rule_pattern.len() - 1]
        } else {
            rule_pattern
        };

        // Get the file name (excluding the path)
        let file_name = path.split('/').last().unwrap_or(path);

        // Handle wildcard matching
        // Here we implement a simple wildcard matching, supporting * and ?
        self.wildcard_match(file_name, rule_pattern)
    }

    fn wildcard_match(&self, text: &str, pattern: &str) -> bool {
        // Implement simple * and ? wildcard matching
        // * matches any number of characters
        // ? matches a single character

        if pattern.is_empty() {
            return text.is_empty();
        }

        let mut i = 0;
        let mut j = 0;
        let mut star_pos = None;
        let mut match_pos = 0;

        while i < text.len() {
            if j < pattern.len()
                && (pattern.as_bytes()[j] == b'?' || pattern.as_bytes()[j] == text.as_bytes()[i])
            {
                // Character matches or ? matches a single character
                i += 1;
                j += 1;
            } else if j < pattern.len() && pattern.as_bytes()[j] == b'*' {
                // * can match 0 or more characters
                star_pos = Some(j);
                match_pos = i;
                j += 1;
            } else if star_pos.is_some() {
                // Backtrack: use * to match more characters
                j = star_pos.unwrap() + 1;
                match_pos += 1;
                i = match_pos;
            } else {
                return false;
            }
        }

        // Handle all * at the end of the pattern
        // They can match any number of characters, including none
        while j < pattern.len() && pattern.as_bytes()[j] == b'*' {
            j += 1;
        }

        j == pattern.len()
    }
}

pub fn filter_entries_by_config(
    entries: Vec<DirEntry>,
    config: &TreeConfig,
    gitignore_filter: Option<&GitignoreFilter>,
) -> Result<Vec<DirEntry>, TreeError> {
    let mut filtered = Vec::new();

    for entry in entries {
        if !config.show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }

        let file_type = entry
            .file_type()
            .map_err(|e| TreeError::GetFileType(entry.path().display().to_string(), e))?;

        if config.dirs_only && !file_type.is_dir() {
            continue;
        }

        if let Some(filter) = gitignore_filter {
            if filter.should_ignore(entry.path().as_path(), file_type.is_dir()) {
                continue;
            }
        }

        filtered.push(entry);
    }

    Ok(filtered)
}
