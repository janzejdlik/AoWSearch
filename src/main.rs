use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    let input_search = input("Enter Input Search: ");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("AoW.txt")
        .expect("Failed to open AoW.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read AoW.txt");

    let lines: Vec<&str> = contents.lines().collect();
    let mut found = false;

    for (i, line) in lines.iter().enumerate() {
        if line.contains(&input_search) {
            found = true;
            let chapter_name = get_chapter_name(&lines, i);
            let chapter_number = get_chapter_number(&lines, i);
            if chapter_name != "Unknown" && chapter_number != "Unknown" {
                println!("{} {}", chapter_name, chapter_number);
            } else {
                println!("{} {}", "Unknown", "Unknown");
            }
            if let Some(sentence_number) = get_sentence_number(line) {
                println!("{}", sentence_number);
            }
            println!("{}", line);
        }
    }

    if !found {
        println!("Input search sentence not found");
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_chapter_name<'a>(lines: &'a [&'a str], index: usize) -> &'a str {
    for i in (0..index).rev() {
        if lines[i].starts_with("I.") {
            return "I. Laying Plans";
        } else if lines[i].starts_with("II.") {
            return "II. Waging War";
        } else if lines[i].starts_with("III.") {
            return "III. Attack by Stratagem";
        } else if lines[i].starts_with("IV.") {
            return "IV. Tactical Dispositions";
        } else if lines[i].starts_with("VI.") {
            return "VI. Weak Points and Strong";
        } else if lines[i].starts_with("VII.") {
            return "VII. Maneuvering";
        } else if lines[i].starts_with("VIII.") {
            return "VIII. Variation in Tactics";
        } else if lines[i].starts_with("IX.") {
            return "IX. The Army on the March";
        } else if lines[i].starts_with("X.") {
            return "X. Terrain";
        } else if lines[i].starts_with("XI.") {
            return "XI. The Nine Situations";
        } else if lines[i].starts_with("XII.") {
            return "XII. The Attack by Fire";
        } else if lines[i].starts_with("XIII.") {
            return "XIII. The Use of Spies";
        }
    }
    "Unknown"
}

fn get_chapter_number<'a>(lines: &'a [&'a str], index: usize) -> &'a str {
    for i in (0..index).rev() {
        if lines[i].starts_with("I.") || lines[i].starts_with("II.") || lines[i].starts_with("III.") || lines[i].starts_with("IV.") || lines[i].starts_with("VI.") || lines[i].starts_with("VII.") || lines[i].starts_with("VIII.") || lines[i].starts_with("IX.") || lines[i].starts_with("X.") || lines[i].starts_with("XI.") || lines[i].starts_with("XII.") || lines[i].starts_with("XIII.") {
            return &lines[i][0..4];
        }
    }
    "Unknown"
}

fn get_sentence_number(line: &str) -> Option<usize> {
    let parts: Vec<_> = line.split_whitespace().collect();
    if parts.len() > 1 && parts[0].chars().all(char::is_numeric) {
        parts[0].parse().ok()
    } else {
        None
    }
}
