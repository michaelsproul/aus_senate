use util::*;

pub fn senate_quota(num_ballots: Frac, num_senators: u32) -> Frac {
    (num_ballots / frac!(num_senators + 1)).floor() + frac!(1)
}
