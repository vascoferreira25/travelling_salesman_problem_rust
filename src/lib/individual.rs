use super::city;
use city::City;
use city::Route;

extern crate rand;
use rand::prelude::*;

/// An individual that will run a route
#[derive(Debug, Clone)]
pub struct Individual {
    route: Route,
    total_distance: f64,
    fitness: f64,
    normalized_fitness: f64,
}

impl Individual {
    /// Create an individual with a given route
    pub fn new(route: Route) -> Individual {
        Individual {
            route,
            total_distance: 0.0,
            fitness: 0.0,
            normalized_fitness: 0.0,
        }
    }

    pub fn get_route(&self) -> &Route {
        &self.route
    }

    pub fn set_route(&mut self, new_route: Route) {
        self.route = new_route
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
    pub fn update(&mut self, max_fitness: f64) {
        self.total_distance = self.route.total_distance();
        self.fitness = 1.0 / (1.0 + self.total_distance.powf(8.0));
        self.normalized_fitness = self.fitness / max_fitness;
    }

    /// combine a pair of individuals to generate a new route
    pub fn crossover(parent_1: &Individual, parent_2: &Individual) -> Route {

        let p_1 = parent_1.get_route().get_cities();
        let p_2 = parent_2.get_route().get_cities();

        let mut rng = rand::thread_rng();
        let split_index: usize = rng.gen_range(0, p_1.len());

        let mut new_route: Vec<City> = Vec::new();
        let mut cur_index = 0;
        while cur_index < split_index {
            new_route.push(p_1[cur_index].clone());
            cur_index += 1;
        }

        for i in 0..p_2.len() {
            let cur_city = &p_2[i];
            if !new_route.iter().any(|city| &city == &cur_city) {
                new_route.push(cur_city.clone());
            }
        }

        Route::new(new_route)
    }
    
    /// Mutate the individual
    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();

        for _ in 0..self.route.get_cities().len() {
            let r_prob: f64 = rng.gen();

            if r_prob < mutation_rate {
                let c_1: usize = rng.gen_range(0, self.route.get_cities().len());
                let c_2: usize = rng.gen_range(0, self.route.get_cities().len());
                self.route.swap(c_1, c_2);
            }
        }
    }

    pub fn print_route(&self) {
        let cities = self.route.get_cities();
        for i in 0..cities.len() {
            println!("City {}: {}", i, cities[i].get_name());
        }
        println!("Total distance: {}", self.route.total_distance());
    }
}
