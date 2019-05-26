use self::FirstPref::*;
use super::ballot_munge::BallotMunge;
use crate::ballot::*;
use crate::candidate::*;
use crate::group::*;

/// The Red vs Blue munger strengthens the stranglehold of the two major parties.
///
/// It does this by ensuring that every ballot preferences both major parties.
#[derive(Debug, Default)]
pub struct RedVsBlue {
    liberal_group: &'static str,
    labor_group: &'static str,
    num_green: usize,
    num_labor: usize,
    num_liberal: usize,
    num_other: usize,
    num_hits: usize,
}

impl RedVsBlue {
    pub fn new(state: &str) -> Self {
        let (liberal_group, labor_group) = match state {
            "NSW" => ("F", "N"),
            "QLD" => ("G", "D"),
            "SA" => ("H", "B"),
            _ => panic!("unsupported state: {}", state),
        };

        Self {
            liberal_group,
            labor_group,
            ..Self::default()
        }
    }
}

enum FirstPref {
    Green,
    Labor,
    Liberal,
    Other,
}

fn categorise(ballot: &Ballot, candidates: &CandidateMap) -> FirstPref {
    ballot
        .prefs
        .iter()
        .flat_map(|id| {
            let candidate = candidates.get(id)?;
            match candidate.party.as_str() {
                "Labor" | "Australian Labor Party" => Some(Labor),
                "Liberal" | "The Nationals" | "Liberal National Party of Queensland" => {
                    Some(Liberal)
                }
                "The Greens" => Some(Green),
                _ => None,
            }
        })
        .next()
        .unwrap_or(Other)
}

impl BallotMunge for RedVsBlue {
    // me: *slaps roof of function*
    //     you can fit so many flimsy assumptions in this bad boi
    fn munge(&mut self, ballot: &mut Ballot, groups: &[Group], candidates: &CandidateMap) {
        let groups_to_pref = match categorise(ballot, candidates) {
            Labor => {
                self.num_labor += 1;
                vec![self.liberal_group]
            }
            Liberal => {
                self.num_liberal += 1;
                vec![self.labor_group]
            }
            Green => {
                self.num_green += 1;
                vec![self.labor_group, self.liberal_group]
            }
            Other => {
                self.num_other += 1;
                // Alternate between preferencing libs and labor
                if self.num_other % 2 == 0 {
                    vec![self.labor_group, self.liberal_group]
                } else {
                    vec![self.liberal_group, self.labor_group]
                }
            }
        };

        let extra_candidates = groups_to_pref
            .into_iter()
            .flat_map(|group_name| {
                let group = groups.iter().find(|g| g.name == group_name).unwrap();
                // Take only candidate IDs we haven't already voted for (quadratic time)
                group
                    .candidate_ids
                    .iter()
                    .filter(|cid| !ballot.prefs.contains(cid))
                    .cloned()
            })
            .collect::<Vec<_>>();

        if !extra_candidates.is_empty() {
            self.num_hits += 1;
            ballot.prefs.extend(extra_candidates);
        }
    }
}
