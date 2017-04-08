#![feature(conservative_impl_trait)]

// Optionally use the system allocator on OS X to enable memory profiling.
#![cfg_attr(feature = "osx_mem_profile", feature(alloc_system))]
#[cfg(feature = "osx_mem_profile")]
extern crate alloc_system;

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate csv;
extern crate rustc_serialize;
extern crate gmp;
#[macro_use]
extern crate text_io;

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
