//
// hollow-knight-save-converter
//
// Copyright 2025 Manos Pitsidianakis
//
// This file is part of hollow-knight-save-converter.
//
// hollow-knight-save-converter is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// hollow-knight-save-converter is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with hollow-knight-save-converter. If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: EUPL-1.2 OR GPL-3.0-or-later

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file
    #[arg(value_name = "FILE")]
    save_file: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output_file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts encoded save file to plain text json.
    ToJson {
        /// Pretty output.
        #[arg(short, long, default_value = "false")]
        indented: bool,
    },
    /// Converts plain text json to encoded save file.
    FromJson,
}

fn main() {
    let cli = Cli::parse();
    if matches!(cli.output_file.try_exists(), Ok(true)) {
        panic!("Output file already exists, will not overwrite. Aborting");
    }

    let file = std::fs::read(&cli.save_file).expect("Could not read input file");

    match cli.command {
        Commands::ToJson { indented } => {
            let json_map = hollow_knight_save_converter::decode(&file);
            if indented {
                std::fs::write(
                    &cli.output_file,
                    serde_json::to_string_pretty(&json_map).unwrap(),
                )
                .expect("Could not write json output to file");
            } else {
                std::fs::write(&cli.output_file, serde_json::to_string(&json_map).unwrap())
                    .expect("Could not write json output to file");
            }
        }
        Commands::FromJson => {
            use serde_json::{Map, Value};

            let json_map: Map<String, Value> =
                serde_json::from_slice(&file).expect("Input file is not valid JSON");
            let encoded = hollow_knight_save_converter::encode(
                serde_json::to_string(&json_map)
                    .expect("Internal error, could not serialize JSON to string")
                    .into_bytes(),
            );
            std::fs::write(&cli.output_file, &encoded)
                .expect("Could not write encoded output to file");
        }
    }
}
