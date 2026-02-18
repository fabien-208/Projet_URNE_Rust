// l'id des candidats (leur position dans la liste de base)
pub type CandidateId = usize;

// les preferences des votants
#[derive(Debug, Clone)]
pub struct Ballot {
    pub ranking: Vec<CandidateId>,
}

// represente le resultat d'une election à interpréter
#[derive(Debug, Clone)]
pub struct Election {
    pub candidates: Vec<String>, // index = CandidateId
    pub ballots: Vec<Ballot>,
}

// une liste des candidats, du meilleur au pire
pub struct VoteResult {
    pub ranking: Vec<CandidateId>,
}