use util::*;

pub fn senate_quota(num_ballots: u32, num_senators: u32) -> Frac {
    (frac!(num_ballots) / frac!(num_senators + 1)).floor() + frac!(1)
}
