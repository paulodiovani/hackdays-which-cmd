use std::{error::{self}, fs::File, io::{self, Read}};

#[derive(Debug)]
struct UnkownShell;

fn main() -> Result<(), Box<dyn error::Error>> {
    let histfile = history_file().expect("This program supports: `bash`, `zsh`.");
    let lines = read_lines_sorted(histfile)?;

    println!("{}", lines.join("\n"));

    Ok(())
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

fn read_lines_sorted(file: String) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(file).expect("History file not found");
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let contents = String::from_utf8_lossy(&buf);

    let mut lines: Vec<String> = contents.lines().map(String::from).collect();
    lines.sort();

    Ok(lines)
}
