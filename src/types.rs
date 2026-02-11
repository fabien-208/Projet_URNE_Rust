pub type CandidateId = usize;

#[derive(Debug, Clone)]
pub struct Ballot {
    pub ranking: Vec<CandidateId>,
}

#[derive(Debug, Clone)]
pub struct Election {
    pub candidates: Vec<String>, // index = CandidateId
    pub ballots: Vec<Ballot>,
}

pub struct VoteResult {
    pub ranking: Vec<CandidateId>,
}