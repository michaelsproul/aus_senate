use util::*;

pub type ExhaustedVotes = BTreeMap<usize, (usize, Frac)>;

#[derive(Debug, Serialize)]
pub struct ExhaustedVoteRow {
    pub round: usize,
    pub ballots_exhausted: usize,
    /// Numerator of the value of votes exhausted at this round.
    pub value_exhausted_num: String,
    /// Denominator of the value of votes exhausted at this round.
    pub value_exhausted_den: String,
}

pub fn write_out(exhausted_votes: &ExhaustedVotes, filename: &str) -> csv::Result<()> {
    let mut wtr = csv::Writer::from_path(filename)?;

    for (&round, &(ballots_exhausted, ref vote_value)) in exhausted_votes {
        wtr.serialize(ExhaustedVoteRow {
            round,
            ballots_exhausted,
            value_exhausted_num: format!("{}", vote_value.get_num()),
            value_exhausted_den: format!("{}", vote_value.get_den()),
        })?;
    }
    wtr.flush()?;

    Ok(())
}
