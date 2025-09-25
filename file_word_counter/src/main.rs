use std::fs;
use std::env;
use std::process;

fn word_frequency(string: &str, file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("File not Found");

    let mut result = 0;

    for word in contents.split_whitespace() {
        if string == word {
            result += 1;
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

     if args.len() != 3 {
        eprintln!();
        eprintln!("Arguments provided: {}", args.len() - 1);
        eprintln!("Arguments Expected: 2");
        eprintln!();
        eprintln!("Usage:");
        eprintln!(" cargo run -- <word-to-search> <file-path>");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  cargo run -- name Cargo.toml");
        process::exit(1);
    }

    let word = &args[1];
    let file_path = &args[2];


    let answer = word_frequency(&word, &file_path);

    print!("{} is present {} times in the file.", word, answer)
}
