use crate::types::{Election, Ballot};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn parse(filepath: &str) -> io::Result<Election>{
    let mut liste_ballots: Vec<Ballot> = Vec::new();
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let liste_candidats: Vec<String> = buffer.trim_end().split(';').map(String::from).collect();
    buffer.clear();

    while reader.read_line(&mut buffer)? > 0 {
        print!("{}", buffer);

        buffer.clear();
        break;
    }

    return Ok(Election {
        candidates: liste_candidats,
        ballots: liste_ballots
    })

}
