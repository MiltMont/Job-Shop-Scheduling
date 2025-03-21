use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("The file {src} does not exists")]
#[diagnostic(
    code(file::error::no_such_file),
    help("Create the file first or copy a correct path")
)]
pub struct FileError {
    pub src: PathBuf,
}
