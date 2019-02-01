#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rand;
extern crate petgraph;
extern crate slotmap;
extern crate fnv;

pub mod modularity;
pub use self::modularity::Modularity;

pub mod io;
