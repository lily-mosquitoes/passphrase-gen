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

use serde::Deserialize;
use std::{
    path::PathBuf,
    collections::HashMap,
};

#[derive(Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Entry {
    pub word: String,
    pub meaning: Option<String>,
    pub example_usage: Option<String>,
}

pub fn word_list(file: PathBuf) -> Result<HashMap<u32, Entry>, anyhow::Error> {
    let mut index: u32 = 1;

    let mut reader = csv::Reader::from_path(file)?;

    let mut map = HashMap::<u32, Entry>::new();
    for entry in reader.deserialize() {
        map.insert(index, entry?);
        index += 1;
    }

    // if there are no records in the table
    if index < 2 {
        return Err(anyhow::Error::msg("expected at least 1 record in the table"));
    }

    Ok(map)
}
