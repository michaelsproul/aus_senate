use ballot::MultiBallot;
use std::error::Error;

pub use self::BallotParseErr::*;

pub enum BallotParseErr {
    InvalidBallot(String),
    InputError(Box<Error>),
}

/// This type is yielded from iterators used during ballot parsing.
///
/// It allows us to capture GVT multi-votes, and handle the two different types of errors:
///     1. Ballot parsing errors, which are recoverable (skip the ballot).
///     2. IO errors, CSV parsing errors, which are not recoverable (stop the algorithm).
pub type IOBallot = Result<MultiBallot, BallotParseErr>;
