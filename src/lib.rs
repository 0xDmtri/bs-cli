#![allow(clippy::clippy::new_without_default)]
#![allow(clippy::clippy::clippy::single_match)]

pub mod pricer;
pub mod types;

mod ivol;
pub use ivol::implied_vol;
