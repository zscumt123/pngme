#![allow(dead_code)]
use std::fs;
use std::str::FromStr;

use args::Cli;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use error::PngError;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;
mod utils;

fn main() -> Result<(), PngError> {
    let args = Cli::parse();

    match &args.command {
        args::PngMeArgs::Encode(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = ChunkType::from_str(cmd.chunk_type.as_str())?;
            let chunk_data = cmd.message.clone().into_bytes();
            let message_chunk = Chunk::new(chunk_type, chunk_data);
            let mut png = Png::from_file(input_file)?;
            png.append_chunk(message_chunk);
            println!("encode ok");
            let out_path = if let Some(p) = &cmd.out_path {
                p
            } else {
                input_file
            };
            fs::write(out_path, png.as_bytes())?;
        }
        args::PngMeArgs::Decode(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = cmd.chunk_type.as_str();
            let png = Png::from_file(input_file)?;
            let message = png.data_string_by_type(chunk_type);
            match message {
                None => {
                    println!("no such message for chunk_type: {}.", chunk_type)
                }
                Some(msg) => {
                    println!("secret msg for {} is: {}", chunk_type, msg)
                }
            }
        }
        args::PngMeArgs::Remove(cmd) => {
            let input_file = &cmd.file_path;
            let chunk_type = cmd.chunk_type.as_str();
            let mut png = Png::from_file(input_file)?;
            png.remove_chunk(chunk_type)?;
            fs::write(input_file, png.as_bytes())?;
            println!("remove chunk type: {}", chunk_type);
        }
        args::PngMeArgs::Print(cmd) => {
            let input_file = &cmd.file_path;
            let png = Png::from_file(input_file)?;
            let v = png.chunks();
            let num = v.len();
            println!("====================all chunk type({num})====================");
            for i in 0..num {
                print!("{}:{};", i + 1, v[i].chunk_type().to_string());
            }
        }
    }
    Ok(())
}
