use super::city::{City, Route};

extern crate rand;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// An individual that will run a given route
#[derive(Debug, Clone)]
pub struct Individual {
    route: Route,
    total_distance: f64,
    fitness: f64,
}

impl Individual {
    /// Create an individual with a given route
    pub fn new(route: Route) -> Individual {
        Individual {
            route,
            total_distance: 0.0,
            fitness: 0.0,
        }
    }

    pub fn get_route(&self) -> &Route {
        &self.route
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness
    }

    /// Shuffle the route
    pub fn shuffle_route(&mut self, rng: &mut ChaCha20Rng) {
        self.route.shuffle(rng);
    }

    /// Update the total distance and fitness values
    pub fn update(&mut self) {
        self.total_distance = self.route.total_distance();
        self.fitness = 1.0 / (1.0 + self.total_distance.powf(8.0));
    }

    /// Mutate the individual
    pub fn mutate(&mut self, rng: &mut ChaCha20Rng, mutation_rate: f64) {
        for _ in 0..self.route.get_cities().len() {
            let r_prob: f64 = rng.gen();

            if r_prob < mutation_rate {
                let c_1: usize = rng.gen_range(0, self.route.get_cities().len());
                let c_2: usize = rng.gen_range(0, self.route.get_cities().len());
                self.route.swap(c_1, c_2);
            }
        }
    }

    /// Print the individual route and total distance
    pub fn print(&self) {
        println!("----------------------------------------------");
        println!("{:^46}", "Individual Info");
        println!("----------------------------------------------");
        println!("- Route:");
        let cities = self.route.get_cities();
        for i in 0..cities.len() {
            println!("    - City {}: {}", i, cities[i].get_name());
        }
        println!("- Total distance: {}", self.route.total_distance());
        println!("- Fitness: {}", self.fitness);
        println!("----------------------------------------------");
    }

    /// combine a pair of individuals to generate a new route
    pub fn crossover(rng: &mut ChaCha20Rng,
                     parent_1: &Individual,
                     parent_2: &Individual) -> Individual {

        let p_1 = parent_1.get_route().get_cities();
        let p_2 = parent_2.get_route().get_cities();

        let split_index: usize = rng.gen_range(0, p_1.len());

        let mut new_route: Vec<City> = Vec::new();
        for i in 0..split_index {
            new_route.push(p_1[i].clone());
        }

        for i in 0..p_2.len() {
            let cur_city = &p_2[i];
            if !new_route.contains(&cur_city) {
                new_route.push(cur_city.clone());
            }
        }

        Individual::new(Route::new(new_route))
    }
}
