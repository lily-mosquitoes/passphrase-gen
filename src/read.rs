use serde::Deserialize;
use std::{
    path::PathBuf,
    collections::HashMap,
};
use cli_table::{Table, Color};
use std::fmt::Display;

fn display_meaning(value: &Option<String>) -> impl Display {
    value.to_owned().unwrap_or("".to_string())
}

fn display_usage(value: &Option<String>) -> impl Display {
    let mut s = value.to_owned().unwrap_or("".to_string());
    // format bold
    s = s.replace("<b>", "\x1B[1;32m");
    s = s.replace("</b>", "\x1B[0m");
    // format italic
    s = s.replace("<i>", "\x1B[1;32m");
    s = s.replace("</i>", "\x1B[0m");
    // format underlined
    s = s.replace("<ins>", "\x1B[1;32m");
    s = s.replace("</ins>", "\x1B[0m");
    // format strikethrough
    s = s.replace("<del>", "\x1B[1;32m");
    s = s.replace("</del>", "\x1B[0m");

    s
}

#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Table)]
pub struct WordEntry {
    #[table(title = "Word", color = "Color::Green", bold)]
    pub word: String,
    #[table(title = "Meaning", color = "Color::Red", display_fn = "display_meaning")]
    pub meaning: Option<String>,
    #[table(title = "Usage", display_fn = "display_usage")]
    pub example_usage: Option<String>,
}

pub fn word_list(file: PathBuf) -> Result<HashMap<u32, WordEntry>, csv::Error> {
    let mut index: u32 = 1;
    let mut map = HashMap::<u32, WordEntry>::new();

    let mut reader = csv::Reader::from_path(file)?;

    for entry in reader.deserialize() {
        map.insert(index, entry?);
        index += 1;
    }

    Ok(map)
}
