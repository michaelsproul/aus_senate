use std::collections::BTreeMap;
use std::cmp::{Ordering, min};
use std::cmp::Ordering::*;

use ballot::*;
use candidate::*;
use group::Group;
use std::error::Error;

pub use self::BallotParseErr::*;
pub use self::InvalidBallotErr::*;
pub use self::ChoiceConstraint::*;
pub use self::CountConstraint::*;

#[derive(Debug)]
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
    InvalidStrict,
    EmptyBallot
}

/// This type is yielded from iterators used during ballot parsing.
///
/// It allows us to capture GVT multi-votes, and handle the two different types of errors:
///     1. Ballot parsing errors, which are recoverable (skip the ballot).
///     2. IO errors, CSV parsing errors, which are not recoverable (stop the algorithm).
pub type IOBallot = Result<Ballot, BallotParseErr>;

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
    // Preferring below the line votes is codified in Section 269(2) of the Electoral Act.
    pub fn official() -> Constraints {
        Constraints {
            choice: PreferBelow,
            counts: vec![MinAbove(1), MinBelow(6)]
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
                MinAbove(min) => Constraints::check_min(vote.len(), min, InvalidMinAbove)?,
                MaxAbove(max) => Constraints::check_max(vote.len(), max, InvalidMaxAbove)?,
                _ => (),
            }
        }
        Ok(vote)
    }

    fn check_below(&self, vote: PrefMap) -> Result<PrefMap, BallotParseErr> {
        for &count_constraint in &self.counts {
            match count_constraint {
                MinBelow(min) => Constraints::check_min(vote.len(), min, InvalidMinBelow)?,
                MaxBelow(max) => Constraints::check_max(vote.len(), max, InvalidMaxBelow)?,
                _ => (),
            }
        }
        Ok(vote)
    }
}

fn remove_repeats_and_gaps<T>((mut map, cutoff): BallotRes<T>)
    -> Result<BTreeMap<u32, T>, BallotParseErr>
{
    // Search for a gap in the order of preferences.
    let missing_pref = map.keys().zip(1..).find(|&(&pref, idx)| pref != idx).map(|(_, idx)| idx);

    // Cut-off at the minimum of the provided cutoff (for doubled prefs) and any missing pref.
    let new_cutoff = match (cutoff, missing_pref) {
        (Some(prev), Some(new)) => Some(min(prev, new)),
        (x @ Some(_), _) | (_, x) => x
    };

    if let Some(cut) = new_cutoff {
        map.split_off(&cut);
    }

    if map.len() > 0 {
        Ok(map)
    } else {
        Err(InvalidBallot(EmptyBallot))
    }
}

pub fn parse_ballot_str(pref_string: &str, groups: &[Group], candidates: &[CandidateId], constraints: &Constraints)
-> IOBallot {
    let mut above_str: Vec<&str> = pref_string.split(',').collect();
    let below_str = above_str.split_off(groups.len());

    let above_the_line = create_group_pref_map(above_str, groups)
        .and_then(remove_repeats_and_gaps)
        .and_then(|v| constraints.check_above(v))
        .map(flatten_group_pref_map);
    let below_the_line = create_pref_map(below_str, candidates)
        .and_then(remove_repeats_and_gaps)
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

/// Ballot parse result including a map, and an optional preference cut off.
type BallotRes<T> = (BTreeMap<u32, T>, Option<u32>);

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
    -> Result<BallotRes<&'a [CandidateId]>, BallotParseErr>
{
    let group_candidates = |idx| {
        let group: &'a Group = &groups[idx];
        group.candidate_ids.as_slice()
    };
    create_map(prefs, group_candidates)
}

fn create_pref_map(prefs: Vec<&str>, candidates: &[CandidateId])
    -> Result<BallotRes<CandidateId>, BallotParseErr>
{
    create_map(prefs, |idx| candidates[idx])
}

fn create_map<F, T>(prefs: Vec<&str>, func: F) -> Result<BallotRes<T>, BallotParseErr>
    where F: Fn(usize) -> T
{
    let mut map = BTreeMap::new();
    let mut pref_cutoff = None;

    for (index, &raw_pref) in prefs.iter().enumerate() {

        let pref = match raw_pref {
            "" => continue,
            "*" | "/" => 1,
            _ =>  raw_pref.parse::<u32>().map_err(|_| InvalidBallot(InvalidCharacter))?
        };

        let value = func(index);
        let prev_value = map.insert(pref, value);

        // If a preference is repeated, we ignore that preference and any
        // higher numbered preferences.
        // Sections 268A(2)(b)(i) and 269(1A)(b)(i).
        if prev_value.is_some() {
            pref_cutoff = Some(match pref_cutoff {
                Some(cutoff) => min(cutoff, pref),
                None => pref
            });
        }
    }

    Ok((map, pref_cutoff))
}

#[cfg(test)]
mod test {
    use super::remove_repeats_and_gaps;
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    #[test]
    fn remove_gaps() {
        let mut pref_map = BTreeMap::from_iter((1..10).zip(1..10));
        pref_map.insert(11, 11);

        assert_eq!(remove_repeats_and_gaps((pref_map.clone(), None)).unwrap().len(), 9);
        assert_eq!(remove_repeats_and_gaps((pref_map.clone(), Some(10))).unwrap().len(), 9);
        assert_eq!(remove_repeats_and_gaps((pref_map.clone(), Some(5))).unwrap().len(), 4);
    }
}
