use std::{io::stdin, time::Instant};

use english_trie::Trie;

const WORDS: &str = include_str!("./466k_english_words.txt");

fn main() {
    let mut trie = Trie::new();

    let start = Instant::now();
    WORDS.lines().for_each(|word| {
        trie.add_word(word);
    });
    let load_duration = start.elapsed();

    let stdin = stdin();

    println!(
        "Loaded {} words in {:?}",
        WORDS.lines().count(),
        load_duration
    );

    loop {
        let mut input = String::new();
        let Ok(_) =  stdin.read_line(&mut input) else {
            break;
        };

        if let Some(index) = input.find('\n') {
            input.remove(index);
        }

        println!("Getting suggestions for {}", input);

        let start = Instant::now();
        let suggestions = trie.get_suggestions(&input);
        let suggestion_duration = start.elapsed();

        println!("{}", suggestions.join("\n"));
        println!(
            "Calculated {} suggestions in {:?}",
            suggestions.len(),
            suggestion_duration
        );
    }
}
