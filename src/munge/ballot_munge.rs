use crate::ballot::Ballot;
use crate::candidate::*;
use crate::group::Group;
use std::fmt::Debug;

pub trait BallotMunge: Debug {
    /// Transform a ballot in a hideous undemocratic fashion!
    fn munge(&mut self, ballot: &mut Ballot, groups: &[Group], candidates: &CandidateMap);
}
