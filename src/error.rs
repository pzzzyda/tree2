#[derive(thiserror::Error, Debug)]
pub enum TreeError {
    #[error("failed to open directory `{0}`: {1}")]
    FailedToReadDir(String, #[source] std::io::Error),

    #[error("failed to read directory entry in `{0}`: {1}")]
    FailedToReadDirEntry(String, #[source] std::io::Error),

    #[error("failed to get file type of `{0}`: {1}")]
    FailedToGetFileType(String, #[source] std::io::Error),

    #[error("failed to get metadata of `{0}`: {1}")]
    FailedToGetMetadata(String, #[source] std::io::Error),
}
