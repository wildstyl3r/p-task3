//! Configuration structures.

use clap::Parser;
#[derive(Parser, Debug)]
/// Type to store the arguments passed to the main.
pub struct Args {
    /// The requested number of nulls in hashes.
    #[arg(short = 'N', long, default_value_t = 0)]
    pub nulls: usize,
    /// The requested number of hashes to find.
    #[arg(short = 'F', long, default_value_t = 3)]
    pub find: usize
}