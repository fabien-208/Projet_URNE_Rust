use crate::types::{Election, Ballot, CandidateId};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

/// Analyse le fichier de données et construit l'objet `Election`.
/// Rejette INTÉGRALEMENT le fichier et soulève une Erreur si le format n'est pas respecté.
pub fn parse(filepath: &str) -> io::Result<Election> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // 1. Lecture stricte de l'en-tête
    let header_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Fichier totalement vide."))??;
    let candidates: Vec<String> = header_line.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

    // S'il n'y a pas de candidats, c'est une erreur d'entrée.
    if candidates.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "En-tête invalide ou aucun candidat détecté."));
    }

    let mut candidate_map: HashMap<&str, usize> = HashMap::new();
    for (i, name) in candidates.iter().enumerate() {
        if candidate_map.contains_key(name.as_str()) {
             return Err(io::Error::new(io::ErrorKind::InvalidData, "Doublon de candidat trouvé dans l'en-tête."));
        }
        candidate_map.insert(name, i);
    }

    let mut ballots: Vec<Ballot> = Vec::with_capacity(50_000_000); 

    // 2. Lecture stricte des bulletins
    for line_result in lines {
        let line_content = line_result?;
        let trimmed_line = line_content.trim();
        
        if trimmed_line.is_empty() { continue; } // Tolérance pour les sauts de lignes vides en fin de fichier

        let mut ranking: Vec<CandidateId> = Vec::with_capacity(candidates.len());

        for part in trimmed_line.split('>') {
            let name = part.trim();
            if name.is_empty() { 
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Format de bulletin invalide (syntaxe '>').")); 
            }

            if let Some(&index) = candidate_map.get(name) {
                let id = index as CandidateId;
                if ranking.contains(&id) { 
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Un candidat apparaît plusieurs fois dans le même bulletin.")); 
                }
                ranking.push(id);
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Candidat inconnu détecté : {}", name)));
            }
        }

        if ranking.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Un bulletin est vide."));
        }

        ranking.shrink_to_fit();
        ballots.push(Ballot { ranking });
    }

    // Sécurité supplémentaire : si le fichier ne contenait que des candidats mais aucun bulletin
    if ballots.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Aucun bulletin de vote n'a été trouvé dans le fichier."));
    }

    Ok(Election { candidates, ballots })
}