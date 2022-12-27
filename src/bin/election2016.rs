#[macro_use]
extern crate log;
extern crate aus_senate;

use aus_senate::cli;
use aus_senate::munge::{BallotMunge, RedVsBlue};
use aus_senate::{election2016, exhausted_votes};
use std::env;
use std::error::Error;

fn main_with_result() -> Result<(), Box<Error>> {
    env_logger::init()?;

    let mut args = cli::Options::from_args();
    let num_candidates = args.num_candidates.unwrap_or(12);
    let mut mungers = args.mungers.take();

    let election_result = election2016::run(
        &args.candidates_file,
        &args.prefs_file,
        &args.state,
        num_candidates,
        &[],
        &mut mungers[..],
    )?;

    println!("=== Elected ===");
    for &(ref c, ref votes) in &election_result.senators {
        println!(
            "{} {} ({}) [{} votes]",
            c.other_names, c.surname, c.party, votes
        );
    }

    if election_result.tied {
        println!("Tie for the last place");
    }

    exhausted_votes::write_out(
        &election_result.stats.exhausted_votes,
        "results/exhausted.csv",
    )?;

    Ok(())
}

fn main() {
    if let Err(e) = main_with_result() {
        error!("Error: {:?}", e);
    }
}
