use std::fs::{self, OpenOptions};
use std::io::Write;

use clap::{arg_enum, Parser};
use regex::Regex;

arg_enum! {
    #[derive(PartialEq, Debug, Clone)]
    pub enum Mode {
        Missing,
        Present,
        Always,
    }
}

/// A utility for appending text to files based on simple conditional logic that
/// is often desirable (but complex) in shell scripts.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the file you want to append to.
    #[clap(short, long, value_parser)]
    path: String,

    /// Text to append to the end of the file if conditions are met.
    #[clap(short, long, value_parser)]
    append: String,

    /// (Optional) Path of the file to check. Defaults to [`path`].
    #[clap(short, long, value_parser)]
    check_path: Option<String>,

    /// Pattern to match against. Defaults to [`append`].
    #[clap(long, value_parser)]
    pattern: Option<String>,

    /// (Optional) Mode for conditional logic.
    #[clap(short, long, value_parser, default_value_t = Mode::Missing)]
    mode: Mode,
}

fn contains_or_matches(pattern: &str, haystack: &str) -> bool {
    let regex = Regex::new(pattern);
    match regex {
        Ok(regex) => regex.is_match(haystack),
        Err(_) => haystack.contains(pattern),
    }
}

fn main() {
    let args = Args::parse();
    let check_path = match args.check_path {
        Some(check_path) => check_path,
        None => args.path.clone(),
    };
    let pattern = match args.pattern {
        Some(pattern) => pattern,
        None => args.append.clone(),
    };
    let contents = fs::read_to_string(check_path).expect("Could not read the specified file");
    let perform_append = match args.mode {
        Mode::Missing => !contains_or_matches(pattern.as_str(), contents.as_str()),
        Mode::Present => contains_or_matches(pattern.as_str(), contents.as_str()),
        Mode::Always => true,
    };
    if perform_append {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(args.path)
            .unwrap();

        writeln!(file, "{}", args.append).expect("Could not write to the specified file");
    }
}
