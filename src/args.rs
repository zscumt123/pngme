use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Subcommand, Debug)]
pub enum PngMeArgs {
    /// encode: hide secret into png / encode <file_path> <chunk_type> <message> <out_file_path>
    Encode(EncodeArgs),
    /// decode: get secret message by chunk type / decode <file_path> <chunk_type>
    Decode(DecodeArgs),
    /// remove: remove secret message by chunk type / remove <file_path> <chunk_type>
    Remove(RemoveArgs),
    /// print: print all chunk type / print <file_path>
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub out_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}
