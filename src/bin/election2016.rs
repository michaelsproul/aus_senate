#[macro_use]
extern crate log;
extern crate aus_senate;

use aus_senate::munge::{BallotMunge, RedVsBlue};
use aus_senate::{election2016, exhausted_votes};
use std::env;
use std::error::Error;

fn main_with_result() -> Result<(), Box<Error>> {
    env_logger::init()?;

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 && args.len() != 5 {
        println!("Usage: ./election2016 <candidates file> <prefs file> <state> [num candidates]");
        Err("invalid command line arguments.".to_string())?;
    }

    let candidates_file_name = &args[1];
    let prefs_file_name = &args[2];
    let state = &args[3];
    let num_candidates = match args.get(4) {
        Some(x) => x.parse::<usize>()?,
        None => 12,
    };

    //let mut mungers = vec![Box::new(RedVsBlue::new(state)) as Box<BallotMunge>];
    let mut mungers = vec![];

    let election_result = election2016::run(
        candidates_file_name,
        prefs_file_name,
        state,
        num_candidates,
        &[],
        &mut mungers[..],
    )?;

    println!("Yikes: {:#?}", mungers);

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
