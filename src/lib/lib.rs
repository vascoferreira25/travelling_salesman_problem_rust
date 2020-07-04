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

use city::{City, Route};
use individual::Individual;

extern crate rand;
use rand::prelude::*;

pub fn start(size: usize, mutation_rate: f64, elitism_size: usize, epochs: usize, cities: Vec<City>) {

    // Create Population
    let mut population: Vec<Individual> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        // Each individual has a cloned version of the cities list
        // and only then will it shuffle to create a random route.
        // TODO: change the route to take a reference
        let route = Route::new(cities.clone());
        
        let mut new_individual = Individual::new(route, mutation_rate);
        new_individual.shuffle_route();
        population.push(new_individual);
    }

    // Epochs
    let mut best_individual: Individual = population[0].clone();
    
    for epoch in 0..epochs {

        // Calculate the sum of the fitness of all the individuals
        let mut fitness_sum = 0.0;
        for individual in &population {
            fitness_sum += individual.get_fitness();
        }

        // Update the individuals.
        for individual in &mut population {
            individual.update(fitness_sum);
        }

        // Set the best individual
        for individual in &population {
            if individual.get_fitness() > best_individual.get_fitness() {
                best_individual = individual.clone();
            }
        }

        // Sort the population based on fitness
        population
            .sort_by(|a, b| 
                     b.get_normalized_fitness()
                     .partial_cmp(&a.get_normalized_fitness()).unwrap());

        // Elitism
        let mut selected: Vec<Individual> = Vec::new();
   
        for i in 0..elitism_size {
            selected.push(population[i].clone());
        }

        // Choose the best individuals more oftem as they have greater
        // normalized fitness.
        for _ in 0..(population.len() - elitism_size) {
            let chosen = population
                .choose_weighted(&mut rng,
                                 |a| a.get_fitness()).unwrap();
            selected.push(chosen.clone());
        }

        population = selected;

        // Crossover and Mutate
        for i in 0..population.len() {
             
            let r_1: usize = rng.gen_range(0, population.len());
            let r_2: usize = rng.gen_range(0, population.len());
            
            let parent_1 = &population[r_1];
            let parent_2 = &population[r_2];

            let crossover = Individual::crossover(&parent_1, &parent_2);
            population[i].set_route(crossover);
            population[i].mutate();
        }

        println!("Epoch {}/{}", epoch, epochs);
    }

    println!("Best individual: {:?}", best_individual);
}
