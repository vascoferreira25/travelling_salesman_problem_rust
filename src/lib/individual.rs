use super::city;
use city::City;
use city::Route;

extern crate rand;
use rand::prelude::*;

/// An individual that will run a route
#[derive(Debug, Clone)]
pub struct Individual {
    route: Route,
    mutation_rate: f64,
    total_distance: f64,
    fitness: f64,
    normalized_fitness: f64,
}

impl Individual {
    /// Create an individual with a given route
    pub fn new(route: Route, mutation_rate: f64) -> Individual {
        Individual {
            route,
            mutation_rate,
            total_distance: 0.0,
            fitness: 0.0,
            normalized_fitness: 0.0,
        }
    }

    pub fn get_route(&self) -> &Route {
        &self.route
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness
    }

    pub fn get_normalized_fitness(&self) -> f64 {
        self.normalized_fitness
    }

    /// Shuffle the route
    pub fn shuffle_route(&mut self) {
        self.route.shuffle();
    }

    /// Update the individual data and calculate the fitness values
    pub fn update(&mut self, fitness_sum: f64) {
        self.total_distance = self.route.total_distance();
        self.fitness = 1.0 / (1.0 + self.total_distance.powf(8.0));
        self.normalized_fitness = self.fitness / fitness_sum;
    }

    /// Set a new route
    pub fn set_new_route(&mut self, new_route: Vec<City>) {
        self.route = Route::new(new_route);
    }
    
    /// Mutate the individuals in the population
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let r_prob: f64 = rng.gen();

        if r_prob < self.mutation_rate {
            let c_1: usize = rng.gen_range(0, self.route.get_cities().len());
            let c_2: usize = rng.gen_range(0, self.route.get_cities().len());

            self.route.swap(c_1, c_2);
        }
    }
}
