//! Manage cities and coordinates.

extern crate rand;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// A coordinate is a point in a 2d plane that has a `x` and `y` values.
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    x: f64,
    y: f64,
}

impl Coordinate {
    /// Create a new instance of a `Coordinate` by specifying an `x` and `y`
    /// values
    pub fn new(x: f64, y: f64) -> Coordinate {
        Coordinate {x: x, y: y}
    }

    /// Calculate the distance between two `Coordinates`
    pub fn distance_to(&self, coord: &Coordinate) -> f64 {
        ((self.x - coord.x).abs().powf(2.0) + (self.y - coord.y).abs().powf(2.0))
            .sqrt()
    }
}

/// A `City` is defined a point in a 2d plane. It has a `name` and
/// `coordinates`
#[derive(Debug, Clone, PartialEq)]
pub struct City {
    name: String,
    coordinates: Coordinate,
}

impl City {
    pub fn new(name: String, coordinates: Coordinate) -> City {
        City { name, coordinates }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Calculate the distance between two `Cities` using each city's
    /// coordinates.
    pub fn distance_to(&self, city: &City) -> f64 {
        self.coordinates.distance_to(&city.coordinates)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    cities: Vec<City>,
}

impl Route {
    pub fn new(cities: Vec<City>) -> Route {
        Route { cities }
    }

    pub fn get_cities(&self) -> &Vec<City> {
        &self.cities
    }

    /// Swap cities
    pub fn swap(&mut self, city_1: usize, city_2: usize) {
        let temp = self.cities[city_1].clone();
        self.cities[city_1] = self.cities[city_2].clone();
        self.cities[city_2] = temp;
    }

    /// Calculate the total distance between all the cities in the route.
    pub fn total_distance(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..(self.cities.len() - 1) {
            let city_1 = &self.cities[i];
            let city_2 = &self.cities[i + 1];

            total += city_1.distance_to(city_2);
        }

        // Calculate the distance between the last city and the first to
        // complete the full circle.
        total += &self.cities[self.cities.len()-1].distance_to(&self.cities[0]);

        total
    }

    /// Shuffles the route randomly
    pub fn shuffle(&mut self, rng: &mut ChaCha20Rng) {
        self.cities.shuffle(rng);
    }
}
