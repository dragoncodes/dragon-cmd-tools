use colored::Colorize;
use regex::Regex;
use std::io::Read;

fn main() {
    let mut input = String::new();

    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Error reading input");

    let original_search_phrase = std::env::args().nth(1);

    let original_search_phrase = original_search_phrase.expect("Empty search phrase");

    let search_phrase = Regex::new(&original_search_phrase.to_lowercase()).unwrap();

    input.lines().for_each(|line| {
        let line = line.to_lowercase().to_owned();

        if let Some(m) = search_phrase.find(&line) {
            let match_start = m.start();

            if match_start == 0 {
                let original_search_phrase_len = original_search_phrase.len();
                let start = &line[0..original_search_phrase_len];
                let end = &line[original_search_phrase_len..];

                println!("{}{}", start.truecolor(255, 125, 121), end);
            } else {
                println!(
                    "{}{}{}",
                    &line[0..match_start],
                    &line[match_start..m.end()].truecolor(255, 125, 121),
                    &line[m.end()..line.len()]
                )
            }
        }
    });
}
