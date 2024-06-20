use colored::Colorize;
use regex::Regex;
use std::io::Read;

fn main() {
    let mut input = String::new();

    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Error reading input");

    let search_phrase = std::env::args().nth(1);

    let search_phrase = search_phrase.expect("Empty search phrase");

    let search_phrase = Regex::new(&search_phrase).unwrap();

    input.lines().for_each(|line| {
        if let Some(m) = search_phrase.find(line) {
            let match_start = m.start();

            if match_start == 0 {
                let start = &line[..(match_start)];
                let end = &line[(match_start)..];

                println!("{}{}", start.red(), end);
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
