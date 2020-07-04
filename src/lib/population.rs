//! Create and update Populations of Individuals

extern crate rand;
use rand::prelude::*;

use super::city::{City, Route};
use super::individual::Individual;

/// Create populations of individuals
#[derive(Debug)]
pub struct Population {
    individuals: Vec<Individual>,
    best_individual: Individual,
}

impl Population {
    pub fn new(cities: Vec<City>, size: usize) -> Population {

        let mut individuals: Vec<Individual> = Vec::new();

        for _ in 0..size {
            // Each individual has a cloned version of the cities list
            // and only then will it shuffle to create a random route.
            // TODO: change the route to take a reference
            let route = Route::new(cities.clone());

            let mut new_individual = Individual::new(route);
            new_individual.shuffle_route();
            individuals.push(new_individual);
        }

        let best_individual = individuals[0].clone();
        
        Population {
            individuals,
            best_individual
        }
    }

    pub fn get_best_individual(&self) -> &Individual {
        &self.best_individual
    }
    
    /// Update all the individuals of the population - complexity `O(2n)`
    ///
    /// This approach loops two times over the individuals vector:
    ///
    /// 1. to calculate the `fitness_sum`,
    /// 2. to update the individuals.
    fn update(&mut self) {
        // Calculate the sum of the fitness of all the individuals
       
        let mut max_fitness = 0.0;
        for individual in &self.individuals {
            let fitness = individual.get_fitness();
            // if fitness > max_fitness {
            //     max_fitness = fitness;
            // }
            max_fitness += fitness;
        }

        // Update the individuals.
        for individual in &mut self.individuals {
            individual.update(max_fitness);
        }
    }

    fn select_best(&mut self) {
        let mut current_best: &Individual = &self.best_individual;
        
        for individual in &self.individuals {
            if individual.get_fitness() > current_best.get_fitness() {
                current_best = individual;
            }
        }

        self.best_individual = current_best.clone();
    }

    /// Sort the population based on the normalized fitness
    fn sort_population(&mut self) {
        self.individuals
            .sort_by(|a, b| 
                     b.get_fitness()
                     .partial_cmp(&a.get_fitness()).unwrap());
    }

    /// Perform roulette selection on the population
    ///
    /// This will select the best performing individuals more often then the
    /// less performing ones.
    fn selection(&mut self, elitism_size: usize) {
        let mut selected: Vec<Individual> = Vec::new();

        // Elitism
        for i in 0..elitism_size {
            selected.push(self.individuals[i].clone());
        }

        let mut rng = rand::thread_rng();
        // Choose the best individuals more often as they have greater
        // normalized fitness. Using `normalized_fitness` leads to non-finite
        // boundaries error for the rand crate.
        for _ in 0..(self.individuals.len() - elitism_size) {
            let chosen = self.individuals
                .choose_weighted(&mut rng,
                                 |a| a.get_fitness()).unwrap();
            selected.push(chosen.clone());
        }

        self.individuals = selected;
    }

    /// Perform a crossover in each individual of the population
    ///
    /// TODO: Create a function the generates a vec of pairs of individuals to
    /// use as parents
    fn crossover_and_mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();
       
        // Crossover and Mutate
        for i in 0..self.individuals.len() {
           
            let r_1: usize = rng.gen_range(0, self.individuals.len());
            let r_2: usize = rng.gen_range(0, self.individuals.len());
            
            let parent_1 = &self.individuals[r_1];
            let parent_2 = &self.individuals[r_2];
 
            let crossover = Individual::crossover(parent_1, parent_2);
            self.individuals[i].set_route(crossover);
            self.individuals[i].mutate(mutation_rate);
        }
    }

    pub fn simulate(&mut self, epochs: usize, elitism_size: usize, mutation_rate: f64) {

        self.update();
        for epoch in 0..epochs {
            self.sort_population();
            self.selection(elitism_size);
            self.crossover_and_mutate(mutation_rate);
            self.update();
            self.select_best();
            
            println!("Epoch: {}/{} - Best Distance: {}", epoch, epochs, self.best_individual.get_route().total_distance());
        }
    }
}
