use std::{cmp, collections::HashMap, error, fs::File, io::{self, Read}, u32};

const DEFAULT_COUNT: usize = 5;

#[derive(Debug)]
struct UnkownShell;

#[derive(Debug)]
struct CommandUsage(String, u32);

fn main() -> Result<(), Box<dyn error::Error>> {
    let count = DEFAULT_COUNT;

    let histfile = history_file().expect("This program supports: `bash`, `zsh`.");
    let lines = read_lines_sorted(histfile)?;
    let lines = cleanup(lines);
    let lines = sort(lines);

    let table = build_command_table(lines);

    // println!("{}", lines.join("\n"));
    // println!("{:?}", table);

    print_command_table(table, count);

    Ok(())
}

fn build_command_table(lines: Vec<String>) -> Vec<CommandUsage> {
    let mut hash: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let count = hash.entry(line).or_insert(0);
        *count += 1;
    }

    let mut commands: Vec<CommandUsage> = hash.iter().map(|(k, v)| CommandUsage(k.to_string(), *v)).collect();
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

fn history_file() -> Result<String, UnkownShell> {
    let home = env!("HOME");
    let shell = env!("SHELL");

    match shell {
        "/bin/bash" => Ok(format!("{}/.bash_history", home)),
        "/bin/zsh" => Ok(format!("{}/.zsh_history", home)),
        _ => Err(UnkownShell),
    }
}

fn print_command_table(table: Vec<CommandUsage>, count: usize) {
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
