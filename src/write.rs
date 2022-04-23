use std::collections::HashMap;
use rand::{
    Rng,
    thread_rng,
    distributions::Uniform,
};
use cli_table::{Table, Color, WithTitle};
use crate::read::Entry;

fn display_meaning(value: &Option<String>) -> String {
    value.to_owned().unwrap_or("".to_string())
}

fn display_usage(value: &Option<String>) -> String {
    let mut s = value.to_owned().unwrap_or("".to_string());
    // format bold
    s = s.replace("<b>", "\x1B[1;32m");
    s = s.replace("</b>", "\x1B[0m");
    // format italic
    s = s.replace("<i>", "\x1B[3;32m");
    s = s.replace("</i>", "\x1B[0m");
    // format underlined
    s = s.replace("<ins>", "\x1B[4;32m");
    s = s.replace("</ins>", "\x1B[0m");
    // format strikethrough
    s = s.replace("<del>", "\x1B[9;32m");
    s = s.replace("</del>", "\x1B[0m");

    s
}

#[derive(Table)]
pub struct Word {
    #[table(title = "Word", color = "Color::Green", bold)]
    pub word: String,
}

#[derive(Table)]
pub struct WordMeaning {
    #[table(title = "Word", color = "Color::Green", bold)]
    pub word: String,
    #[table(title = "Meaning", color = "Color::Red")]
    pub meaning: String,
}

#[derive(Table)]
pub struct WordMeaningUsage {
    #[table(title = "Word", color = "Color::Green", bold)]
    pub word: String,
    #[table(title = "Meaning", color = "Color::Red")]
    pub meaning: String,
    #[table(title = "Usage")]
    pub example_usage: String,
}

#[derive(Table)]
pub struct WordUsage {
    #[table(title = "Word", color = "Color::Green", bold)]
    pub word: String,
    #[table(title = "Usage")]
    pub example_usage: String,
}

pub fn table(words: &HashMap<u32, Entry>, number_of_words: u8) -> Result<(), anyhow::Error> {
    let mut rng = thread_rng();
    let range = Uniform::new_inclusive(1, words.len());
    let mut words_dice = (&mut rng).sample_iter(range);

    let entry_format = words.get(&1)
        .expect("Index out of range for word list (please report, this is a bug)");

    println!("Chosen words:\n");
    match (&entry_format.meaning, &entry_format.example_usage) {
        (Some(_), Some(_)) => {
            let mut chosen_list = Vec::<WordMeaningUsage>::new();
            for _ in 1..=number_of_words {
                let index = words_dice.next()
                    .expect("Failed to get next word value (please report, this is a bug)") as u32;
                let chosen_word = words.get(&index)
                    .expect("Index out of range for word list (please report, this is a bug)");

                chosen_list.push(WordMeaningUsage {
                    word: chosen_word.word.to_owned(),
                    meaning: display_meaning(&chosen_word.meaning),
                    example_usage: display_usage(&chosen_word.example_usage),
                });
            }

            println!("{:}", chosen_list.with_title().display().unwrap());
        },
        (Some(_), None) => {
            let mut chosen_list = Vec::<WordMeaning>::new();
            for _ in 1..=number_of_words {
                let index = words_dice.next()
                    .expect("Failed to get next word value (please report, this is a bug)") as u32;
                let chosen_word = words.get(&index)
                    .expect("Index out of range for word list (please report, this is a bug)");

                chosen_list.push(WordMeaning {
                    word: chosen_word.word.to_owned(),
                    meaning: display_meaning(&chosen_word.meaning),
                });
            }

            println!("{:}", chosen_list.with_title().display().unwrap());
        },
        (None, None) => {
            let mut chosen_list = Vec::<Word>::new();
            for _ in 1..=number_of_words {
                let index = words_dice.next()
                    .expect("Failed to get next word value (please report, this is a bug)") as u32;
                let chosen_word = words.get(&index)
                    .expect("Index out of range for word list (please report, this is a bug)");

                chosen_list.push(Word {
                    word: chosen_word.word.to_owned(),
                });
            }

            println!("{:}", chosen_list.with_title().display().unwrap());
        },
        (None, Some(_)) => {
            let mut chosen_list = Vec::<WordUsage>::new();
            for _ in 1..=number_of_words {
                let index = words_dice.next()
                    .expect("Failed to get next word value (please report, this is a bug)") as u32;
                let chosen_word = words.get(&index)
                    .expect("Index out of range for word list (please report, this is a bug)");

                chosen_list.push(WordUsage {
                    word: chosen_word.word.to_owned(),
                    example_usage: display_usage(&chosen_word.example_usage),
                });
            }

            println!("{:}", chosen_list.with_title().display().unwrap());
        },
    }

    Ok(())
}
