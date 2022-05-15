// Copyright (C) 2022  Lílian Ferreira de Freitas
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod read;
pub mod write;

use clap::{Parser, CommandFactory, ErrorKind};
use std::path::PathBuf;

/// Script for generating xkcd-style passphrases from a given word list
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Word list file (must be `.csv`)
    #[clap(short, long)]
    file: PathBuf,
    /// Number of words to generate (1 ≤ n ≤ 255)
    #[clap(short, long, default_value_t = 4)]
    number_of_words: u8,
}

fn min_entropy(possible_words: f64, number_of_words: u8) -> f64 {
    let mut max_possible_passwords = 1_f64;
    for n in 1..=number_of_words {
        max_possible_passwords *= possible_words / n as f64;
    }

    max_possible_passwords.log2()
}

fn main() {
    let args = Cli::parse();
    let mut cmd = Cli::command();

    if args.number_of_words < 1 {
        cmd.error(ErrorKind::ValueValidation,
            "`number_of_words` must be at least 1")
            .exit()
    }

    let words = match read::word_list(args.file) {
        Ok(words) => words,
        Err(error) => cmd.error(ErrorKind::Io,
            error.to_string())
            .exit()
    };

    match write::table(&words, args.number_of_words) {
        Ok(()) => (),
        Err(error) => cmd.error(ErrorKind::Io,
            error.to_string())
            .exit()
    }

    let min_entropy = min_entropy(words.len() as f64, args.number_of_words);
    println!("\nPassphrase entropy ≥ {:.2}", min_entropy);
}
