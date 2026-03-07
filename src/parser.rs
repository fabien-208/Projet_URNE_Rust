use crate::types::{Election, Ballot, CandidateId};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

pub fn parse(filepath: &str) -> io::Result<Election> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().enumerate();

    let header_tuple = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Fichier vide"))?;
    let header = header_tuple.1?;

    let candidates: Vec<String> = header.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    let mut candidate_map: HashMap<&str, usize> = HashMap::new();
    for (i, name) in candidates.iter().enumerate() {
        candidate_map.insert(name, i);
    }

    // On alloue la capacité direct pour éviter les copies en RAM
    let mut ballots: Vec<Ballot> = Vec::with_capacity(50_000_000); 

    for (_line_index, line_result) in lines {
        let line_content = line_result?;
        let trimmed_line = line_content.trim();
        if trimmed_line.is_empty() { continue; }

        let mut ranking: Vec<CandidateId> = Vec::with_capacity(candidates.len());
        let mut is_valid = true;

        for part in trimmed_line.split('>') {
            let name = part.trim();
            if name.is_empty() { is_valid = false; break; }

            if let Some(&index) = candidate_map.get(name) {
                let id = index as u8; // 🎯 Conversion ici
                if ranking.contains(&id) { is_valid = false; break; }
                ranking.push(id);
            } else {
                is_valid = false; break;
            }
        }

        if is_valid && !ranking.is_empty() {
            ranking.shrink_to_fit();
            ballots.push(Ballot { ranking });
        }
    }
    Ok(Election { candidates, ballots })
}