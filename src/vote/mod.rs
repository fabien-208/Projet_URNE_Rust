/// Interface définissant le comportement standard que chaque algorithme de vote doit respecter.
/// Le trait exige l'implémentation de `Sync` pour permettre l'exécution multithreadée via Rayon.
pub trait VotingAlgorithm: Sync {
    /// Retourne le nom de l'algorithme tel qu'il doit être affiché dans la sortie standard.
    fn name(&self) -> String;
    
    /// Exécute les calculs de l'algorithme sur l'élection donnée et retourne le classement final.
    fn compute(&self, election: &crate::types::Election) -> crate::types::VoteResult;
}

// Déclaration de tous les sous-modules contenant les différentes méthodes de vote
pub mod plurality;
pub mod trs;
pub mod irv;
pub mod borda;
pub mod bucklin;
pub mod baldwin;
pub mod copeland;
pub mod copeland_borda;
pub mod schulze;
pub mod smith_irv;