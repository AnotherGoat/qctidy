/// Contains the library's domain structures used to represent a `Graph`.
pub(crate) mod circuit;
pub(crate) mod graph;
/// Contains general math utilities.
pub(crate) mod math;
/// General number conversion utilities.
pub(crate) mod number;
pub(crate) mod projection;

#[cfg(test)]
mod math_tests;
#[cfg(test)]
mod number_tests;
