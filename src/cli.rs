use crate::munge::{BallotMunge, RedVsBlue};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// Path to CSV file with candidate details.
    candidates_file: String,
    /// Path to CSV file of preference orderings.
    prefs_file: String,
    /// Which state are we running an election for?
    state: String,
    /// Number of senate positions to fill.
    num_candidates: Option<usize>,
    /// Modifications to apply to ballots (in order).
    #[structopt(parse(try_from_str = "parse_munger"))]
    mungers: Vec<MungerType>,
}

fn parse_munger(name: &str) -> Result<MungerType, ()> {
    match name {
        "red-vs-blue" | "rvb" => Ok(MungerType::RedVsBlueT),
        _ => Err(()),
    }
}

#[derive(Clone, Copy)]
pub enum MungerType {
    RedVsBlue,
}

impl MungerType {
    pub fn instantiate(self, state: &str) -> Box<BallotMunge> {
        match self {
            MungerType::RedVsBlue => Box::new(RedVsBlue::new(state)),
        }
    }
}
