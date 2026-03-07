mod parser;
mod types;
mod vote; 

use std::env;
use std::process;
use crate::vote::VotingAlgorithm;
use std::time::Instant;

fn main() {
    // Gestion des Arguments
    let args: Vec<String> = env::args().collect();

    // Le sujet demande d'afficher une erreur explicite sur la sortie d'erreur
    if args.len() < 2 {
        eprintln!("❌ Erreur : Veuillez spécifier un fichier de données.");
        eprintln!("Usage : cargo run --release -- <fichier_donnees>");
        process::exit(1);
    }

    let filename = &args[1];
    
    // Info pour le développeur sur stderr
    eprintln!("📂 Lecture du fichier : {}", filename);

    // Parsing
    let election = match parser::parse(filename) {
        Ok(e) => {
            eprintln!("✅ Chargé : {} candidats, {} bulletins.", e.candidates.len(), e.ballots.len());
            e
        },
        Err(e) => {
            // Message d'erreur explicite requis
            eprintln!("❌ Erreur fatale lecture fichier : {}", e);
            process::exit(1);
        }
    };

    //  Liste des Algorithmes
    //  L'ordre doit être respecté : Plurality, TRS, IRV, Borda, Bucklin, Baldwin, Copeland, Copeland+Borda, Schulze, Smith+IRV
    let algorithms: Vec<Box<dyn VotingAlgorithm>> = vec![
        Box::new(vote::plurality::Plurality),
        Box::new(vote::trs::Trs),
        Box::new(vote::irv::IRV),
        Box::new(vote::borda::Borda),
        Box::new(vote::bucklin::Bucklin),
        Box::new(vote::baldwin::Baldwin),
        Box::new(vote::copeland::Copeland),
        Box::new(vote::copeland_borda::CopelandBorda),
        Box::new(vote::schulze::Schulze),
        Box::new(vote::smith_irv::SmithIRV),
    ];

    // Exécution
    for algo in algorithms {
        // On chronomètre pour la frime
        let start = Instant::now();
        
        let result = algo.compute(&election);
        
        let duration = start.elapsed();
        eprintln!("   ⏱  {} calculé en {:?}", algo.name(), duration);

        // SORTIE OFFICIELLE-
        // Syntaxe : "[nom]: [vainqueur]\n"

        
        if let Some(&winner_id) = result.ranking.first() {
            // On récupère le nom du candidat via son ID
            println!("{}: {}", algo.name(), election.candidates[winner_id as usize]);
        } else {
            // Cas théorique impossible avec un fichier valide non-vide
            eprintln!(" Pas de vainqueur trouvé pour {}", algo.name());
        }
    }
}