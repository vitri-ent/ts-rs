#![warn(clippy::pedantic, clippy::nursery)]

use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

use clap::Parser;
use color_eyre::{owo_colors::OwoColorize, Result};

mod args;
mod cargo;
mod metadata;
mod path;

use args::Args;
use metadata::{Metadata, FILE_NAME};

const BLANK_LINE: [u8; 2] = [b'\n', b'\n'];
const NOTE: &[u8; 109] = b"// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n";

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let metadata_path = args.output_directory.join(FILE_NAME);
    if metadata_path.exists() {
        fs::remove_file(&metadata_path)?;
    }

    if args.merge_files && args.generate_index_ts {
        eprintln!(
            "{} --index is not compatible with --merge",
            "Error:".red().bold()
        );

        return Ok(());
    }

    cargo::invoke(&args)?;

    let metadata_content = fs::read_to_string(&metadata_path)?;
    let metadata = Metadata::try_from(&*metadata_content)?;

    let demand_unique_names = args.merge_files || args.generate_index_ts;

    if !demand_unique_names || metadata.is_empty() {
        return Ok(());
    }

    if metadata.has_naming_collisions() {
        metadata.report_naming_collisions();

        eprintln!(
            "{} due to the naming collisions listed above, generating an index.ts file is not possible",
            "Error:".red().bold()
        );

        return Ok(());
    }

    let index_path = args.output_directory.join("index.ts");

    if index_path.exists() {
        fs::remove_file(&index_path)?;
    }

    let mut index = OpenOptions::new()
        .create(true)
        .append(true)
        .open(index_path)?;

    index.write_all(NOTE)?;

    if args.generate_index_ts {
        for path in metadata.export_paths() {
            index.write_fmt(format_args!("\nexport * from {path:?};"))?;
        }

        return Ok(());
    }

    if args.merge_files {
        for path in metadata.export_paths() {
            let path = path::absolute(args.output_directory.join(path))?;
            let mut file = OpenOptions::new().read(true).open(&path)?;

            let mut buf = Vec::with_capacity(file.metadata()?.len().try_into()?);
            file.read_to_end(&mut buf)?;

            let Some((i, _)) = buf.windows(2).enumerate().find(|(_, w)| w == &BLANK_LINE) else {
                continue;
            };

            index.write_all(&buf[i + 1..])?;

            fs::remove_file(path)?;
        }

        path::remove_empty_subdirectories(&args.output_directory)?;

        return Ok(());
    }

    Ok(())
}
