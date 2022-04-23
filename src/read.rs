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
