use super::prelude::*;

/// Below the line voting map. Maps (batch, paper) pairs to preferences.
pub type BelowTheLine = HashMap<(u32, u32), PrefMap>;

#[derive(Deserialize, Debug)]
struct BTLRow {
    candidate_id: u32,
    preference: Option<u32>,
    batch: u32,
    paper: u32,
}

pub fn parse<R: Read>(input: R) -> Result<BelowTheLine, Box<Error>> {
    let mut btl_votes = HashMap::new();
    let mut invalid_votes = HashSet::new();
    let mut reader = ::csv::Reader::from_reader(input);

    for raw_row in reader.deserialize::<BTLRow>() {
        let row = raw_row?;
        let vote_id = (row.batch, row.paper);
        match row.preference {
            Some(pref) => {
                let voter_prefs = btl_votes.entry(vote_id).or_insert_with(BTreeMap::new);
                let prev = voter_prefs.insert(pref, row.candidate_id);
                // Can't assign a single preference to more than one candidate!
                // NOTE: this makes loads of NSW BTL ballots invalid... around 15%. Weird.
                if prev.is_some() {
                    invalid_votes.insert(vote_id);
                }
            }
            None => {
                invalid_votes.insert(vote_id);
            }
        }
    }

    // Remove invalid votes.
    println!("Invalid BTL votes: {}", invalid_votes.len());
    for vote_id in invalid_votes {
        btl_votes.remove(&vote_id);
    }
    println!("Valid BTL votes: {}", btl_votes.len());

    Ok(btl_votes)
}
