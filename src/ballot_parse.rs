use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

use ballot::*;
use candidate::*;
use group::Group;
use std::error::Error;

pub use self::BallotParseErr::*;
pub use self::InvalidBallotErr::*;
pub use self::ChoiceConstraint::*;
pub use self::CountConstraint::*;

pub enum BallotParseErr {
    InvalidBallot(InvalidBallotErr),
    InputError(Box<Error>),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum InvalidBallotErr {
    InvalidCharacter,
    InvalidMinAbove(usize),
    InvalidMaxAbove(usize),
    InvalidMinBelow(usize),
    InvalidMaxBelow(usize),
    InvalidStrict
}

/// This type is yielded from iterators used during ballot parsing.
///
/// It allows us to capture GVT multi-votes, and handle the two different types of errors:
///     1. Ballot parsing errors, which are recoverable (skip the ballot).
///     2. IO errors, CSV parsing errors, which are not recoverable (stop the algorithm).
pub type IOBallot = Result<Ballot<u32>, BallotParseErr>;

#[derive(Clone, Copy)]
pub enum ChoiceConstraint {
    Strict,
    PreferAbove,
    PreferBelow,
}

#[derive(Clone, Copy)]
pub enum CountConstraint {
    MinAbove(usize),
    MaxAbove(usize),
    MinBelow(usize),
    MaxBelow(usize),
}

pub struct Constraints {
    pub choice: ChoiceConstraint,
    pub counts: Vec<CountConstraint>,
}

impl Constraints {
    pub fn strict2016() -> Constraints {
        Constraints {
            choice: Strict,
            counts: vec![MinAbove(6), MinBelow(12)]
        }
    }

    pub fn lax2016() -> Constraints {
        Constraints {
            // Somewhat arbitrary.
            choice: PreferBelow,
            counts: vec![MinAbove(1), MinBelow(1)]
        }
    }

    fn check_cmp<F>(invalid: Ordering, vote_length: usize, val: usize, err: F) -> Result<(), BallotParseErr>
        where F: Fn(usize) -> InvalidBallotErr
    {
        if vote_length.cmp(&val) == invalid {
            Err(InvalidBallot(err(vote_length)))
        } else {
            Ok(())
        }
    }

    fn check_min<F>(vote_length: usize, min: usize, err: F) -> Result<(), BallotParseErr>
        where F: Fn(usize) -> InvalidBallotErr
    {
        // good if: vote_length >= min, bad if: vote_length < min
        Constraints::check_cmp(Less, vote_length, min, err)
    }

    fn check_max<F>(vote_length: usize, max: usize, err: F) -> Result<(), BallotParseErr>
            where F: Fn(usize) -> InvalidBallotErr
    {
        // good if: vote_length <= max, i.e. bad if vote_length > max
        Constraints::check_cmp(Greater, vote_length, max, err)
    }

    /// Validate an above the line vote.
    fn check_above<'a>(&self, vote: GroupPrefMap<'a>) -> Result<GroupPrefMap<'a>, BallotParseErr> {
        for &count_constraint in &self.counts {
            match count_constraint {
                MinAbove(min) => try!(Constraints::check_min(vote.len(), min, InvalidMinAbove)),
                MaxAbove(max) => try!(Constraints::check_max(vote.len(), max, InvalidMaxAbove)),
                _ => (),
            }
        }
        Ok(vote)
    }

    fn check_below(&self, vote: PrefMap) -> Result<PrefMap, BallotParseErr> {
        for &count_constraint in &self.counts {
            match count_constraint {
                MinBelow(min) => try!(Constraints::check_min(vote.len(), min, InvalidMinBelow)),
                MaxBelow(max) => try!(Constraints::check_max(vote.len(), max, InvalidMaxBelow)),
                _ => (),
            }
        }
        Ok(vote)
    }
}

pub fn parse_ballot_str(pref_string: &str, groups: &[Group], candidates: &[CandidateId], constraints: &Constraints)
-> IOBallot {
    let mut above_str: Vec<&str> = pref_string.split(',').collect();
    let below_str = above_str.split_off(groups.len());

    let above_the_line = create_group_pref_map(above_str, groups)
        .and_then(|v| constraints.check_above(v))
        .map(flatten_group_pref_map);
    let below_the_line = create_pref_map(below_str, candidates)
        .and_then(|v| constraints.check_below(v))
        .map(flatten_pref_map);

    match (constraints.choice, above_the_line, below_the_line) {
        (_, Ok(prefs), Err(_)) |
        (_, Err(_), Ok(prefs)) |
        (PreferAbove, Ok(prefs), Ok(_)) |
        (PreferBelow, Ok(_), Ok(prefs)) => {
            Ok(Ballot::single(prefs))
        }
        (Strict, Ok(_), Ok(_)) => {
            Err(InvalidBallot(InvalidStrict))
        }
        (_, Err(e1), Err(_)) => {
            Err(e1)
        }
    }
}

/// Mapping from preferences to candidate IDs (below the line voting).
pub type PrefMap = BTreeMap<u32, CandidateId>;

/// Mapping from preferences to groups of candidates (above the line voting).
pub type GroupPrefMap<'a> = BTreeMap<u32, &'a [CandidateId]>;

pub fn flatten_pref_map(pref_map: PrefMap) -> Vec<CandidateId> {
    pref_map.values().map(|&x| x).collect()
}

pub fn flatten_group_pref_map(group_pref_map: GroupPrefMap) -> Vec<CandidateId> {
    let size = group_pref_map.values().map(|x| x.len()).sum();
    let mut flat = Vec::with_capacity(size);

    for i in group_pref_map.keys() {
        flat.extend_from_slice(group_pref_map[i]);
    }

    flat
}

fn create_group_pref_map<'a>(prefs: Vec<&str>, groups: &'a [Group])
    -> Result<GroupPrefMap<'a>, BallotParseErr>
{
    let group_candidates = |idx| {
        let group: &'a Group = &groups[idx];
        group.candidate_ids.as_slice()
    };
    create_map(prefs, group_candidates)
}

fn create_pref_map(prefs: Vec<&str>, candidates: &[CandidateId])
    -> Result<PrefMap, BallotParseErr>
{
    create_map(prefs, |idx| candidates[idx])
}

fn create_map<F, T>(prefs: Vec<&str>, func: F) -> Result<BTreeMap<u32, T>, BallotParseErr>
    where F: Fn(usize) -> T
{
    let mut map = BTreeMap::new();

    for (index, raw_pref) in prefs.iter().enumerate() {
        if raw_pref.is_empty() {
            continue;
        }
        let pref = try!(raw_pref.parse::<u32>().map_err(|_| InvalidBallot(InvalidCharacter)));
        let value = func(index);
        map.insert(pref, value);
    }

    Ok(map)
}
