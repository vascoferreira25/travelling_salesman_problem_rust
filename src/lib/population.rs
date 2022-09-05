//! Create and update Populations of Individuals

extern crate rand;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::individual;

use super::city::{City, Route};
use super::individual::Individual;

/// Create populations of individuals
#[derive(Debug)]
pub struct Population {
    individuals: Vec<Individual>,
    best_individual: Individual,
    rng: ChaCha20Rng
}

impl Population {
    pub fn new(cities: Vec<City>, size: usize, seed: u64) -> Population {
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mut individuals: Vec<Individual> = Vec::new();

        for _ in 0..size {
            // Each individual has a cloned version of the cities list
            // and only then will it shuffle to create a random route.
            // TODO: change the route to take a reference
            let route = Route::new(cities.clone());

            let mut new_individual = Individual::new(route);
            new_individual.shuffle_route(&mut rng);
            individuals.push(new_individual);
        }

        let best_individual = individuals[0].clone();
        
        Population {
            individuals,
            best_individual,
            rng
        }
    }

    pub fn get_best_individual(&self) -> &Individual {
        &self.best_individual
    }
    
    /// Update all the individuals of the population
    fn update(&mut self) {
        // Select the best individual
        for individual in &mut self.individuals {
            individual.update();
            if individual.get_fitness() > self.best_individual.get_fitness() {
                self.best_individual = individual.clone();
            }
        }
    }

    /// Perform roulette selection on the population
    ///
    /// This will select the best performing individuals more often then the
    /// less performing ones.
    fn selection(&mut self, elitism_size: usize) {
        self.individuals
            .sort_by(|a, b| 
                     b.get_fitness()
                     .partial_cmp(&a.get_fitness()).unwrap());

        let mut selected: Vec<Individual> = Vec::new();
        let mut total_fitness: f64 = 0.0;
        for i in 0..self.individuals.len() {
            total_fitness += self.individuals[i].get_fitness();
            // Elitism
            if i < elitism_size {
                selected.push(self.individuals[i].clone());
            }
        }

        // Choose the best individuals more often as they have greater fitness.
        for _ in 0..(self.individuals.len() - elitism_size) {
            let mut accumulated_weight = 0.0;
            let random_value: f64 = self.rng.gen_range(0.0, total_fitness);

            for individual in &self.individuals {
                accumulated_weight += individual.get_fitness();
                if accumulated_weight > random_value {
                    selected.push(individual.clone());
                    break;
                }
            }
        }

        self.individuals = selected;
    }

    /// Perform a crossover in each individual of the population
    fn crossover_and_mutate(&mut self, mutation_rate: f64) {
        // Crossover and Mutate
        for i in 0..self.individuals.len() {
            let r_1: usize = self.rng.gen_range(0, self.individuals.len());
            let r_2: usize = self.rng.gen_range(0, self.individuals.len());
            
            let parent_1 = &self.individuals[r_1];
            let parent_2 = &self.individuals[r_2];
 
            let mut crossover = Individual::crossover(&mut self.rng, parent_1, parent_2);
            crossover.mutate(&mut self.rng, mutation_rate);
            
            self.individuals[i] = crossover;
        }
    }

    /// Run the simulation
    pub fn simulate(&mut self,
                    epochs: usize,
                    elitism_size: usize,
                    mutation_rate: f64,
                    print_progress: bool) {
        self.update();
        for epoch in 0..epochs {
            self.selection(elitism_size);
            self.crossover_and_mutate(mutation_rate);
            self.update();
            if print_progress {
                println!("Epoch: {}/{} - Best Distance: {}",
                         epoch, epochs, self.best_individual.get_route().total_distance());
            }
        }
    }
}
