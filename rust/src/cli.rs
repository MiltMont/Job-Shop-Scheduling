use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
pub struct CliArgs {
    /// Path to the file containing the problem specification
    #[arg(short, long)]
    pub file: PathBuf,

    /// Display the parsed instance
    #[arg(long = "di")]
    pub display_instance: bool,

    /// Display the solution
    #[arg(long = "ds")]
    pub display_solution: bool,
}
