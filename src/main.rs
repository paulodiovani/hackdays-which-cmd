use clap::Parser;
use std::{cmp, collections::HashMap, error, fs::File, io::{self, Read}, u32};

#[derive(Debug)]
struct UnkownShell;

#[derive(Debug)]
struct CommandTable(String, u32);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number commands to print
    #[arg(short, long, default_value_t = 5)]
    count: u8,

    /// List of commands to ignore
    #[arg(short, long)]
    ignore: Option<String>,

    /// Optional name of command to search
    name: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let count: usize = args.count.into();
    let ignore: Option<String> = args.ignore;

    let histfile = history_file().expect("This program supports: `bash`, `zsh`.");
    let lines = read_lines_sorted(histfile)?;
    let lines = cleanup(lines);
    let lines = filter_ignored(lines, ignore);
    let lines = sort(lines);

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

fn filter_ignored(lines: Vec<String>, ignore: Option<String>) -> Vec<String> {
    let mut filtered_lines: Vec<String> = lines;

    let ignore_list: Vec<&str> = match &ignore {
        Some(text) => text.split(',').collect(),
        None => vec![],
    };

    for word in ignore_list {
        filtered_lines = filtered_lines.iter().filter(|l| !l.starts_with(word)).map(|l| l.to_string()).collect();
    }

    filtered_lines
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

fn sort(lines: Vec<String>) -> Vec<String> {
    let mut lines = lines;
    lines.sort();
    lines
}
