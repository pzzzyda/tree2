pub struct TreeConfig {
    pub max_depth: Option<usize>,
    pub show_hidden: bool,
    pub dirs_only: bool,
    pub with_color: bool,
    pub ascii_only: bool,
    pub sort: bool,
    pub ignore_gitignore: bool,
}

pub enum TreeIndentType {
    Space,
    Vertical,
    Last,
    Continue,
}

impl TreeConfig {
    pub fn get_indent(&self, indent_type: TreeIndentType) -> &'static str {
        match indent_type {
            TreeIndentType::Space => self.get_indent_space(),
            TreeIndentType::Vertical => self.get_indent_vertical(),
            TreeIndentType::Last => self.get_indent_last(),
            TreeIndentType::Continue => self.get_indent_continue(),
        }
    }

    fn get_indent_space(&self) -> &'static str {
        "    "
    }

    fn get_indent_vertical(&self) -> &'static str {
        if self.ascii_only { "|   " } else { "│   " }
    }

    fn get_indent_last(&self) -> &'static str {
        if self.ascii_only {
            "`-- "
        } else {
            "└── "
        }
    }

    fn get_indent_continue(&self) -> &'static str {
        if self.ascii_only {
            "|-- "
        } else {
            "├── "
        }
    }
}
