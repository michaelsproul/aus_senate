// Optionally use the system allocator on OS X to enable memory profiling.
#![cfg_attr(feature = "osx_mem_profile", feature(alloc_system))]
#[cfg(feature = "osx_mem_profile")]
extern crate alloc_system;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate csv;
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
pub mod candidate;
pub mod group;
pub mod ballot;
pub mod voting;
mod vote_map;
pub mod ballot_parse;
pub mod parse;
pub mod senate_result;
pub mod stats;
mod arith;
mod vote_log;
