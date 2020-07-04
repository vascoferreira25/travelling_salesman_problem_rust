//! Create and update Populations of Individuals

extern crate rand;
use rand::prelude::*;

use super::city::{City, Route};
use super::individual::Individual;

/// Create populations of individuals
#[derive(Debug)]
pub struct Population {
    individuals: Vec<Individual>,
    best_individual: Option<Individual>,
}

impl Population {
    pub fn new() -> Population {

        let individuals: Vec<Individual> = Vec::new();

        Population {
            individuals,
            best_individual: None,
        }
    }

    /// Generate random individuals for the population
    pub fn generate_individuals(&mut self, cities: Vec<City>, size: usize, mutation_rate: f64) {
        for _ in 0..size {
            // Each individual has a cloned version of the cities list
            // and only then will it shuffle to create a random route.
            // TODO: change the route to take a reference
            let route = Route::new(cities.clone());

            let mut new_individual = Individual::new(route, mutation_rate);
            new_individual.shuffle_route();
            self.individuals.push(new_individual);
        }
    }

    /// Update all the individuals of the population - `O(2n)`
    ///
    /// This approach loops two times over the individuals vector:
    ///
    /// 1. to calculate the `fitness_sum`,
    /// 2. to update the individuals.
    ///
    /// # Alternative Approach
    ///
    /// Another approach would be to the `Individual` struct to have a method
    /// to loop over the population and then update the fitness of itself.  The
    /// problem with this approach is that you would have to loop with a
    /// `mutable` reference to the vector (as you would want to mutate the
    /// individuals) and then pass an `immutable` reference of the population
    /// to each individual to update the fitness (as you don't want the
    /// individual to mutate the population). This cannot be done as you can't
    /// use `mutable` and `immutable` references at the same time.
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
    /// Hypothetical method to update the fitness:
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
    /// ## Performance Problems
    /// 
    /// Also, you would have less performance as the method `update_fitness` in
    /// the `Individual` struct would have to take a reference to the
    /// population and then loop through all the individuals to calculate the
    /// `fitness_sum` and only then update the fitness. To update all the
    /// individuals it would take `O(n^2)`.
    fn update(&mut self) {
        // Calculate the sum of the fitness of all the individuals
        let mut current_best: Individual;
        match &self.best_individual {
            Some(i) => current_best = i.clone(),
            None => current_best = self.individuals[0].clone()
        }
        
        let mut fitness_sum = 0.0;
        for individual in &self.individuals {
            fitness_sum += individual.get_fitness();
        }

        // Update the individuals.
        for individual in &mut self.individuals {
            individual.update(fitness_sum);
            
            if individual.get_fitness() > current_best.get_fitness() {
                current_best = individual.clone();
            }
        }

        // Select the best individual and sort the population
        self.best_individual = Some(current_best);
        self.sort_population();
    }

    /// Sort the population based on the normalized fitness
    fn sort_population(&mut self) {
        self.individuals
            .sort_by(|a, b| 
                     b.get_normalized_fitness()
                     .partial_cmp(&a.get_normalized_fitness()).unwrap());
    }

    /// Perform roulette selection on the population
    ///
    /// This will select the best performing individuals more often then the
    /// less performing ones.
    fn elitism(&mut self, size: usize) {
        let mut selected: Vec<Individual> = Vec::new();
   
        for i in 0..size {
            selected.push(self.individuals[i].clone());
        }

        // Choose the best individuals more oftem as they have greater
        // normalized fitness.
        for _ in 0..(self.individuals.len() - size) {
            let chosen = self.individuals
                .choose_weighted(&mut rand::thread_rng(),
                                 |a| a.get_normalized_fitness()).unwrap();
            selected.push(chosen.clone());
        }

        self.individuals = selected;
    }

    /// Perform a crossover in each individual of the population
    pub fn crossover(&mut self) {
        for i in 0..self.individuals.len() {
            let parent_1 = self.get_random_individual();
            let parent_2 = self.get_random_individual();
            let r = Individual::crossover(&parent_1, &parent_2);
            self.individuals[i].set_route(r);
        }
    }

}
