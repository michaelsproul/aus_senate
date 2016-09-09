// Optionally use the system allocator on OS X to enable memory profiling.
#![cfg_attr(feature = "osx_mem_profile", feature(alloc_system))]
#[cfg(feature = "osx_mem_profile")]
extern crate alloc_system;

extern crate num;

#[macro_use]
pub mod util;
pub mod candidate;
pub mod group;
pub mod ballot;
pub mod voting;
pub mod vote_map;
pub mod ballot_parse;
