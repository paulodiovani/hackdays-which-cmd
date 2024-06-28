use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let histfile = "/Users/diovani/.zsh_history";

    let mut file = File::open(histfile)?;
    let mut buf = vec![];
    file.read_to_end (&mut buf)?;
    let contents = String::from_utf8_lossy (&buf);

    let mut lines: Vec<&str> = contents.lines().collect();
    lines.sort();

    println!("{}", lines.join("\n"));

    Ok(())
}
