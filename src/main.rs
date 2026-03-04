mod parser;
mod types;
mod vote; 

use std::env;
use std::process;
use crate::vote::VotingAlgorithm;
use std::time::Instant;

fn main() {
    //Gestion des Arguments (Ligne de commande)
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❌ Erreur : Veuillez spécifier un fichier de données.");
        eprintln!("Usage : cargo run --release -- <fichier_donnees>");
        eprintln!("Exemple : cargo run --release -- Data/Data.txt");
        process::exit(1);
    }

    let global_start = Instant::now();

    let filename = &args[1];
    println!("📂 Lecture du fichier : {}", filename);

    // Parsing du fichier
    let election = match parser::parse(filename) {
        Ok(e) => {
            println!("Fichier chargé avec succès !");
            println!("{} candidats, {} bulletins de vote.", e.candidates.len(), e.ballots.len());
            println!("---------------------------------------------------");
            e
        },
        Err(e) => {
            eprintln!("❌ Erreur lors de la lecture du fichier : {}", e);
            process::exit(1);
        }
    };

    // Liste des Algorithmes à tester
    // On met tous tes algos dans un vecteur pour boucler dessus proprement
    let algorithms: Vec<Box<dyn VotingAlgorithm>> = vec![
        Box::new(vote::plurality::Plurality),
        Box::new(vote::borda::Borda),
        Box::new(vote::copeland::Copeland),
        Box::new(vote::copeland_borda::CopelandBorda),
        Box::new(vote::bucklin::Bucklin),
        // Box::new(vote::schulze::Schulze),
    ];

    // Exécution et Affichage des Résultats
    for algo in algorithms {
        run_algorithm(&*algo, &election);
    }

    let global_duration = global_start.elapsed();

    println!("\n----------------------------------------------");
    println!("    TEMPS TOTAL D'EXÉCUTION : {:?}", global_duration);
    println!("----------------------------------------------");
}

// Fonction utilitaire pour lancer un algo et afficher joliment le résultat
fn run_algorithm(algo: &dyn VotingAlgorithm, election: &crate::types::Election) {
    println!("\nMéthode : \x1b[1m{}\x1b[0m", algo.name()); // Nom en gras
    
    // On mesure le temps (optionnel, mais cool pour la frime)
    let start = std::time::Instant::now();
    let result = algo.compute(election);
    let duration = start.elapsed();

    // Récupération du gagnant (le premier du classement)
    if let Some(&winner_id) = result.ranking.first() {
        let winner_name = &election.candidates[winner_id];
        println!("    Vainqueur : \x1b[32m{}\x1b[0m", winner_name); // Vert
    } else {
        println!("    Aucun vainqueur déterminé.");
    }

    // Affichage du classement complet (Top 5 pour ne pas spammer si y'a 100 candidats)
    print!("   Classement : ");
    for (i, &candidate_id) in result.ranking.iter().take(5).enumerate() {
        if i > 0 { print!(" > "); }
        print!("{}", election.candidates[candidate_id]);
    }
    if result.ranking.len() > 5 { print!(" > ..."); }
    println!();
    
    println!("        Temps : {:?}", duration);    
}