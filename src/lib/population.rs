//! Manage populations

extern crate rand;
use rand::prelude::*;

use super::{city, individual};
use city::{City, Route};
use individual::Individual;

/// Create populations of individuals
#[derive(Debug)]
pub struct Population {
    cities: Vec<city::City>,
    individuals: Vec<Individual>,
    best_individual: Option<Individual>,
}

impl Population {
    pub fn new(cities: Vec<City>, mutation_rate: f64, size: usize) -> Population {

        let individuals: Vec<Individual> = Vec::new();

        Population {
            cities,
            individuals,
            best_individual: None,
        }
    }

    /// Generate random individuals for the population
    pub fn generate_individuals(&mut self, size: usize, mutation_rate: f64) {
        for _ in 0..size {
            // Each individual has a cloned version of the cities list
            // and only then will it shuffle to create a random route.
            let route = Route::new(self.cities.clone());

            let mut new_individual = Individual::new(route, mutation_rate);
            new_individual.shuffle_route();
            self.individuals.push(new_individual);
        }
    }

    pub fn select_best_individual(&mut self) {
        // Clone the best individual to compare with individuals in each
        // iteration. If you don't clone the individual and instead use a
        // reference, you will eventually get the object changed over the
        // iterations as you will perform changes on population (mutation and
        // elitism).
     
        let mut current_best: &Individual;
 
        match &self.best_individual {
            Some(i) => current_best = i,
            None => current_best = &self.individuals[0]
        }

        for individual in &self.individuals {
            if individual.get_fitness() > current_best.get_fitness() {
                current_best = &individual
            }
        }
        
        self.best_individual = Some(current_best.clone());
    }

    /// Update all the individuals of the population `O(2n)`
    ///
    /// This approach loops two times over the individuals vector:
    ///
    /// 1. calculate the `fitness_sum`,
    /// 2. update the individuals.
    ///
    /// # Alternative Approach
    /// 
    /// Another approach would be to loop over the individuals and the
    /// `Individual` struct having a method to update the fitness based on the
    /// population. The problem with this approach is that you would have to
    /// loop with a `mutable` reference to the vector (as you would want to
    /// mutate the individuals) and then pass an `immutable` reference of the
    /// population to each individual to update the fitness (as you don't want
    /// the individual to mutate the population). This cannot be done as you
    /// can't use `immutable` and `mutable` references at the same time.
    ///
    /// Hypothetical loop to update the individuals:
    ///
    /// ```ignore
    /// for ind in &mut self.individuals {
    ///      ind.update(&self.individuals);
    ///      // won't compile because `individuals` is borrowed as
    ///      // immutable (&) and mutable (&mut) at the same time
    /// }
    /// ```
    ///
    /// Possible method to update the fitness:
    ///
    /// ```ignore
    /// pub fn update(&self, individuals: &Vec<Individual>) {
    ///     let mut fitness_sum = 0.0;
    ///     for ind in &individuals {
    ///         fitness_sum += ind.get_fitness();
    ///     }
    ///     self.normalized_fitness = 1 / fitness_sum;
    /// }
    /// ```
    ///
    /// Also, you would have decreased performance. As the method
    /// `update_fitness` in the `Individual` struct would have to take a
    /// reference to the population and then loop through all the individuals
    /// to calculate the `fitness_sum` and only then update the fitness, to update
    /// all the individuals it would take `O(n^2)`.
    pub fn update(&mut self) {
        // Calculate the sum of the fitness of all the individuals
        let mut fitness_sum = 0.0;
        for individual in &self.individuals {
            fitness_sum += individual.get_fitness();
        }

        // Update the individuals.
        for individual in &mut self.individuals {
            individual.update(fitness_sum);
        }

        self.sort_population();
    }

    /// Sort the population based on the normalized fitness
    pub fn sort_population(&mut self) {
        self.individuals
            .sort_by(|a, b| 
                     b.get_normalized_fitness()
                     .partial_cmp(&a.get_normalized_fitness()).unwrap());
    }

    /// Perform roulette selection on the population
    ///
    /// This will select the best performing individuals more often then the
    /// less performing ones.
    pub fn elitism(&mut self, size: usize) {
        let mut selected: Vec<Individual> = Vec::new();
   
        for i in 0..size {
            selected.push(self.individuals[i].clone());
        }

        for _ in 0..(self.individuals.len() - size) {
            let chosen = self.individuals
                .choose_weighted(&mut rand::thread_rng(),
                                 |a| a.get_normalized_fitness()).unwrap();
            selected.push(chosen.clone());
        }

        self.individuals = selected;
    }

    /// combine a pair of individuals to create a new one
    pub fn crossover(&mut self) {

        let mut new_individuals: Vec<Individual> = Vec::new();

        for _ in 0..(self.individuals.len()) {
            let parent_1: Vec<City> = self.individuals
                .choose(&mut rand::thread_rng())
                .unwrap()
                .get_route()
                .get_cities();
            let parent_2: Vec<City> = self.individuals
                .choose(&mut rand::thread_rng())
                .unwrap()
                .get_route()
                .get_cities();

            let mut rng = rand::thread_rng();
            let split_index: usize = rng.gen_range(0, self.individuals.len());

            let mut new_route: Vec<City> = Vec::new();

            let mut cur_index = 0;
            while cur_index < split_index {
                new_route.push(parent_1[cur_index].clone());
                cur_index += 1;
            }

            for i in 0..parent_2.len() {
                let cur_city = &parent_2[i];
                if !new_route.iter().any(|city| &city == &cur_city) {
                    new_route.push(cur_city.clone());
                }
            }

            new_individuals.push(Individual::new(Route::new(new_route)));
        }

        self.individuals = new_individuals;
    }

}
