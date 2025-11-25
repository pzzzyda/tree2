#[derive(thiserror::Error, Debug)]
pub enum TreeError {
    #[error("failed to open directory `{0}`: {1}")]
    ReadDir(String, #[source] std::io::Error),

    #[error("failed to read directory entry in `{0}`: {1}")]
    ReadDirEntry(String, #[source] std::io::Error),

    #[error("failed to get file type of `{0}`: {1}")]
    GetFileType(String, #[source] std::io::Error),

    #[error("failed to get metadata of `{0}`: {1}")]
    GetMetadata(String, #[source] std::io::Error),

    #[error("failed to read .gitignore file `{0}`: {1}")]
    ReadGitignore(String, #[source] std::io::Error),
}
