use std::{error::{self}, fs::File, io::{self, Read}};

#[derive(Debug)]
struct UnkownShell;

fn main() -> Result<(), Box<dyn error::Error>> {
    let histfile = history_file().expect("This program supports: `bash`, `zsh`.");
    let lines = read_lines_sorted(histfile)?;
    let lines = cleanup(lines);
    let lines = sort(lines);

    println!("{}", lines.join("\n"));

    Ok(())
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

fn sort(lines: Vec<String>) -> Vec<String> {
    let mut lines = lines;
    lines.sort();
    lines
}

fn read_lines_sorted(file: String) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(file).expect("History file not found");
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    let lines: Vec<String> = contents.lines().map(String::from).collect();

    Ok(lines)
}
