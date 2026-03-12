mod parser;
mod types;
mod vote; 

use std::env;
use std::process;
use crate::vote::VotingAlgorithm;

fn main() {
    // Gestion des Arguments
    let args: Vec<String> = env::args().collect();

    // Le sujet demande d'afficher une erreur explicite sur la sortie d'erreur (stderr)
    if args.len() < 2 {
        eprintln!("Erreur : Veuillez spécifier un fichier de données.");
        process::exit(1);
    }

    let filename = &args[1];

    // Parsing : S'il y a la moindre erreur d'entrée, on l'affiche sur stderr et on coupe tout.
    // AUCUN println! (stdout) ne sera exécuté. Le programme ne panique pas.
    let election = match parser::parse(filename) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur d'entrée : {}", e);
            process::exit(1);
        }
    };

    //  Liste des Algorithmes dans l'ordre strict imposé par le projet
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

    // Exécution 100% silencieuse
    for algo in algorithms {
        let result = algo.compute(&election);
        
        // SORTIE OFFICIELLE (stdout)
        // Syntaxe strictement respectée : "[nom]: [vainqueur]\n"
        if let Some(&winner_id) = result.ranking.first() {
            println!("{}: {}", algo.name(), election.candidates[winner_id as usize]);
        }
    }
}