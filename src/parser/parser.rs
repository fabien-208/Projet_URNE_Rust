use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parser(filepath: &str) -> Election{
    let mut ret = Election;
    let mut liste_ballots: Vec<Ballots>;
    let file = File::open("filepath")?;

    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let liste_candidats: Vec<&str> = buffer.split(';').collect();
    buffer.clear();

    while reader.read_line(&mut buffer)? > 0 {
        print!("{}", buffer);

        buffer.clear();
        break;
    }

    return ret;
    Ok(())

}
