pub mod cli;
mod process;
mod utils;

pub use self::cli::{Base64Format, Opts, OutputFormat, SubCommand, TextSignFormat};
pub use process::{
    process_csv, process_decode, process_encode, process_generate, process_genpass,
    process_text_sign, process_text_verify,
};
pub use utils::*;
