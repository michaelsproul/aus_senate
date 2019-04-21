// Optionally use the system allocator on OS X to enable memory profiling.
#![cfg_attr(feature = "osx_mem_profile", feature(alloc_system))]
#[cfg(feature = "osx_mem_profile")]
extern crate alloc_system;

#[macro_use]
extern crate log;
extern crate csv;
extern crate env_logger;
extern crate gmp;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate unwrap;
extern crate itertools;
extern crate rand;

#[macro_use]
pub mod util;
mod arith;
pub mod ballot;
pub mod ballot_parse;
pub mod candidate;
pub mod group;
pub mod parse;
pub mod senate_result;
pub mod stats;
mod vote_log;
mod vote_map;
pub mod voting;
