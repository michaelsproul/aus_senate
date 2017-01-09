use super::prelude::*;

pub type GVTUsage = HashMap<String, HashMap<String, u32>>;

/// GVT usage parsing.
#[derive(RustcDecodable, Debug)]
struct GVTUsageRow {
    state: String,
    ticket: String,
    group_ab: String,
    group_name: String,
    ticket_votes: u32,
    ticket_percentage: String,
    non_ticket_votes: String,
    non_ticket_percentage: String,
    total_votes: String
}

pub fn parse<R: Read>(input: R) -> Result<GVTUsage, Box<Error>> {
    let mut gvt_usage = HashMap::new();

    let mut reader = ::csv::Reader::from_reader(input);

    for raw_row in reader.decode::<GVTUsageRow>() {
        let row = raw_row?;
        let ticket_map = gvt_usage.entry(row.state).or_insert_with(HashMap::new);
        // Skip ungrouped candidates with 0 vote.
        if &row.ticket == "UG" {
            continue;
        }
        let prev = ticket_map.insert(row.ticket, row.ticket_votes);
        assert!(prev.is_none());
    }

    Ok(gvt_usage)
}
