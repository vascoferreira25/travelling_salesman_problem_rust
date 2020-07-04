//! Genetic Algorithm
//!
//! To run the genetic algorithm to solve the Travelling Salesman Problem, you
//! first have to:
//!
//! - create a vector of cities to find the best route for,
//! - create a population to hold all the individuals,
//! - generate the individuals of the population,
//! - iterate for as many times as you want over the individuals while
//!   updating, crossing over, mutating and selecting the best individuals.
//!
//! ```rust
//! extern crate genetic_algorithm;
//! use genetic_algorithm::{city, population};
//!
//! fn main() {
//!     let cities = vec![
//!         city::City::new(String::from("01"), city::Coordinate::new(-100.0,    0.0)),
//!         city::City::new(String::from("02"), city::Coordinate::new( 100.0,    0.0)),
//!         city::City::new(String::from("03"), city::Coordinate::new(   0.0,  100.0)),
//!         city::City::new(String::from("04"), city::Coordinate::new(   0.0, -100.0)),
//!         city::City::new(String::from("05"), city::Coordinate::new( -90.0,   10.0)),
//!         // more cities...
//!     ];
//!
//!     // Create a population
//!     let mut pop = population::Population::new();
//!     pop.generate_individuals(cities, 100, 0.2);
//!
//!     // TODO: loop over the individuals
//!     // TODO: update the individuals
//!     // TODO: cross over
//!     // TODO: mutate
//!     // TODO: select the best
//! }
//! ```

pub mod city;
pub mod individual;
pub mod population;
