use crate::types::{Election, Ballot};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

pub fn parse(filepath: &str) -> io::Result<Election> {
    // Ouverture du fichier en mode lecture tamponnée (rapide)
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    // On garde une trace du numéro de ligne pour les messages d'erreur (enumerate commence à 0)
    let mut lines = reader.lines().enumerate();

    // --- ÉTAPE 1 : Lecture de l'en-tête (Ligne 1) ---
    // On récupère la première ligne qui contient les noms des candidats
    let header_tuple = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "Fichier vide"))?;
    let header = header_tuple.1?;

    // On nettoie et on stocke les candidats
    let candidates: Vec<String> = header
        .split(|c| c == ';' || c == ',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Création d'un dictionnaire (HashMap) pour trouver l'index d'un candidat instantanément
    let mut candidate_map: HashMap<&str, usize> = HashMap::new();
    for (i, name) in candidates.iter().enumerate() {
        candidate_map.insert(name, i);
    }

    // On prépare le stockage des bulletins. 
    // On met une capacité initiale raisonnable pour éviter trop de réallocations,
    // mais pas trop grosse pour ne pas saturer la RAM si le fichier est petit.
    let mut ballots: Vec<Ballot> = Vec::with_capacity(1_000_000); 

    // --- ÉTAPE 2 : Lecture des votes (Boucle Optimisée) ---
    for (line_index, line_result) in lines {
        let line_content = line_result?;
        let trimmed_line = line_content.trim();
        
        // Si la ligne est vide, on passe à la suivante
        if trimmed_line.is_empty() { continue; }

        let mut ranking: Vec<usize> = Vec::with_capacity(candidates.len());
        
        // OPTIMISATION : On itère directement sur le split (lazy iterator).
        // Cela évite de créer un tableau temporaire de Strings pour chaque ligne.
        // On accepte '>', ';' et ',' comme séparateurs.
        for part in trimmed_line.split(|c| c == '>' || c == ';' || c == ',') {
            let name = part.trim();
            
            // GESTION DES ERREURS DE SYNTAXE (ex: "A>>B" ou "A> >B")
            // Si un segment est vide, ce n'est pas grave, on l'ignore silencieusement.
            if name.is_empty() { continue; }

            if let Some(&index) = candidate_map.get(name) {
                // GESTION DES DOUBLONS (ex: "A>B>A")
                // On vérifie si le candidat est déjà dans le classement de ce bulletin.
                // S'il y est déjà, on ignore cette répétition.
                if !ranking.contains(&index) {
                    ranking.push(index);
                }
            } else {
                // GESTION DES INCONNUS (ex: Vote pour "Toto")
                // C'est la seule erreur qui mérite d'être signalée à l'utilisateur.
                eprintln!("⚠️ Ligne {} : Candidat inconnu '{}' ignoré.", line_index + 1, name);
            }
        }

        // Si le bulletin contient au moins un vote valide, on l'enregistre
        if !ranking.is_empty() {
            ranking.shrink_to_fit(); 
            ballots.push(Ballot { ranking });
        }
    }

    Ok(Election { candidates, ballots })
}