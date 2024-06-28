use clap::Parser;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::{cmp, collections::HashMap, error, fs::File, io::{self, Read}, u32};

#[derive(Debug)]
struct UnkownShell;

#[derive(Debug)]
struct CommandTable(String, u32);

#[derive(Debug)]
enum SearchMethod {
    Exact,
    Fuzzy,
}

#[derive(Parser, Debug)]
#[command(version, about = "Which command I used before to achieve that? 🤔")]
struct Args {
    /// Number commands to print
    #[arg(short, long, default_value_t = 5)]
    count: u8,

    /// Use exact search match
    #[arg(long)]
    exact: bool,

    /// Use fuzzy search match (default)
    #[arg(long)]
    fuzzy: bool,

    /// List of commands to ignore
    #[arg(short, long, value_delimiter = ',')]
    ignore: Vec<String>,

    /// Minimum score for fuzzy search
    #[arg(short, long, default_value_t = 40)]
    score: i64,

    /// Optional search for commands
    search: Vec<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let count: usize = args.count.into();
    let ignore: Vec<String> = args.ignore;
    let score: i64 = args.score;
    let search: Vec<String> = args.search;
    let search_method = if args.exact && !args.fuzzy { SearchMethod::Exact } else { SearchMethod::Fuzzy };

    let histfile = history_file().expect("This program supports: `bash`, `zsh`.");
    let lines = read_lines_sorted(histfile)?;
    let lines = cleanup(lines);
    let lines = filter_ignored(lines, ignore);
    let lines = filter_searched(lines, search, score, search_method);

    let table = build_command_table(lines);

    print_command_table(table, count);
    Ok(())
}

fn build_command_table(lines: Vec<String>) -> Vec<CommandTable> {
    let mut hash: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let count = hash.entry(line).or_insert(0);
        *count += 1;
    }

    let mut commands: Vec<CommandTable> = hash.iter().map(|(k, v)| CommandTable(k.to_string(), *v)).collect();
    commands.sort_by(|a, b| b.1.cmp(&a.1));
    commands
}

fn cleanup(mut lines: Vec<String>) -> Vec<String> {
    // .zsh_history has extra metadata we don't want to use right now
    if env!("SHELL") == "/bin/zsh" {
        lines = cleanup_zsh(lines);
    }

    // remove empty lines
    let lines: Vec<String> = lines.iter().filter(|l| !l.is_empty()).map(|l| l.to_string()).collect();

    lines
}

fn cleanup_zsh(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(';').collect();
        if let Some(part) = parts.get(1) {
            part.to_string()
        } else {
            String::from("")
        }
    }).collect()
}

fn filter_ignored(mut lines: Vec<String>, ignore: Vec<String>) -> Vec<String> {
    for word in ignore {
        lines = lines.iter().filter(|l| !l.starts_with(word.as_str())).map(|l| l.to_string()).collect();
    }

    lines
}

fn filter_searched(lines: Vec<String>, search: Vec<String>, min_score: i64, method: SearchMethod) -> Vec<String> {
    if search.len() == 0 {
        return lines;
    }

    match method {
        SearchMethod::Exact => filter_exact(lines, search),
        SearchMethod::Fuzzy => filter_fuzzy(lines, search, min_score),
    }
}

fn filter_exact(lines: Vec<String>, search: Vec<String>) -> Vec<String> {
    let search: String = search.join(" ");
    let lines = lines.iter().filter(|line| line.contains(search.as_str())).map(String::to_string).collect();

    lines
}

fn filter_fuzzy(lines: Vec<String>, search: Vec<String>, min_score: i64) -> Vec<String> {
    let search: String = search.join(" ");
    let fuzzy_finder = SkimMatcherV2::default();

    let lines = lines.iter().filter(|line| {
        match fuzzy_finder.fuzzy_match(line, search.as_str()) {
            Some(score) => score >= min_score,
            None => false,
        }
    }).map(String::to_string).collect();

    lines
}

fn history_file() -> Result<String, UnkownShell> {
    let home = env!("HOME");
    let shell = env!("SHELL");

    match shell {
        "/bin/bash" => Ok(format!("{}/.bash_history", home)),
        "/bin/zsh" => Ok(format!("{}/.zsh_history", home)),
        _ => Err(UnkownShell),
    }
}

fn print_command_table(table: Vec<CommandTable>, count: usize) {
    let max_count = cmp::min(table.len() / 2, count);

    println!("Here are your {} most used commands:\n", max_count);

    for top in 0..max_count {
        let command = table.get(top).unwrap();
        println!("used {} times\t{}", command.1, command.0);
    }

    println!("\n");
    println!("Here are your {} least used commands:\n", max_count);

    let last = table.len() -1;
    for bottom in ((last - max_count)..last).rev() {
        let command = table.get(bottom).unwrap();
        println!("used {} times\t{}", command.1, command.0);
    }
}

fn read_lines_sorted(file: String) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(file).expect("History file not found");
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    let lines: Vec<String> = contents.lines().map(String::from).collect();

    Ok(lines)
}
