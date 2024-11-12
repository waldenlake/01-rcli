pub mod cli;
mod process;

pub use self::cli::{Opts, OutputFormat, SubCommand};
pub use process::{process_csv, process_decode, process_encode, process_genpass};
