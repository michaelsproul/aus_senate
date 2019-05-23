use ballot::*;
use ballot_parse::*;
use util::*;

pub type BallotErrorMap = HashMap<InvalidBallotErr, u32>;

#[derive(Debug, Default)]
pub struct Stats {
    num_valid_votes: u32,
    invalid_votes: BallotErrorMap,
    /// Map from vote round to number of ballots exhausted during that round (not cumulative) and
    /// their combined value (sum of transfer value).
    pub exhausted_votes: BTreeMap<usize, (usize, Frac)>,
}

impl Stats {
    pub fn new() -> Stats {
        Self::default()
    }

    pub fn record_valid_vote(&mut self, ballot: &Ballot) {
        self.num_valid_votes += ballot.weight();
    }

    pub fn record_invalid_vote(&mut self, err: InvalidBallotErr) {
        let err_count = self.invalid_votes.entry(err.erase_detail()).or_insert(0);
        *err_count += 1;
    }

    pub fn record_exhausted_vote(&mut self, round: usize, transfer_value: &Frac) {
        let &mut (ref mut count, ref mut value) = self
            .exhausted_votes
            .entry(round)
            .or_insert_with(|| (0, frac!(0u64)));
        *count += 1;
        *value += transfer_value;
    }

    pub fn num_total_votes(&self) -> u32 {
        self.num_valid_votes() + self.num_invalid_votes()
    }

    pub fn num_valid_votes(&self) -> u32 {
        self.num_valid_votes
    }

    pub fn num_invalid_votes(&self) -> u32 {
        self.invalid_votes.values().sum()
    }
}

impl InvalidBallotErr {
    pub fn erase_detail(self) -> InvalidBallotErr {
        match self {
            InvalidMinAbove(_) => InvalidMinAbove(0),
            InvalidMaxAbove(_) => InvalidMaxAbove(0),
            InvalidMinBelow(_) => InvalidMinBelow(0),
            InvalidMaxBelow(_) => InvalidMaxBelow(0),
            x => x,
        }
    }
}
