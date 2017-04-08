use util::*;

/// Record of a candidate's vote tally at each iteration of the algorithm.
///
/// Used primarily to break ties.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VoteLog {
    log: Vec<Int>
}

impl VoteLog {
    pub fn new() -> Self {
        VoteLog {
            log: vec![]
        }
    }

    pub fn update_vote(&mut self, idx: usize, vote: Int) {
        if idx < self.log.len() {
            self.log[idx] += vote;
        } else {
            let to_duplicate = idx - self.log.len();
            let dupe = self.maybe_latest().cloned();
            for _ in 0..to_duplicate {
                self.log.push(dupe.clone().unwrap());
            }
            let new_latest = self.maybe_latest().cloned().unwrap_or_else(|| Int::from(0)) + vote;
            self.log.push(new_latest);
            debug_assert_eq!(self.log.len(), idx + 1);
        }
    }

    pub fn maybe_latest(&self) -> Option<&Int> {
        self.log.last()
    }

    pub fn latest(&self) -> &Int {
        self.maybe_latest().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ordering() {
        let mut v1 = VoteLog::new();
        let mut v2 = VoteLog::new();

        // v1 = [15, 16, 19]
        v1.update_vote(0, Int::from(5));
        v1.update_vote(0, Int::from(10));
        v1.update_vote(1, Int::from(1));
        v1.update_vote(2, Int::from(3));
        assert_eq!(v1.latest(), &Int::from(19));

        // v2 = [15, 16, 20]
        v2.update_vote(0, Int::from(15));
        v2.update_vote(1, Int::from(1));
        v2.update_vote(2, Int::from(1));
        v2.update_vote(2, Int::from(3));
        assert_eq!(v2.latest(), &Int::from(20));

        assert!(v1 < v2);
        assert!(v1 != v2);
        assert!(v2 > v1);
    }
}
