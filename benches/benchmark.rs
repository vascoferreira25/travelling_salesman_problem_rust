use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate genetic_algorithm;

use genetic_algorithm::city::{City, Coordinate};
use genetic_algorithm::population::Population;

/// Main function
fn ga_bench() {

    let cities = vec![
        City::new(String::from("01"), Coordinate::new(-100.0,    0.0)),
        City::new(String::from("02"), Coordinate::new( 100.0,    0.0)),
        City::new(String::from("03"), Coordinate::new(   0.0,  100.0)),
        City::new(String::from("04"), Coordinate::new(   0.0, -100.0)),
        City::new(String::from("05"), Coordinate::new( -90.0,   10.0)),
        City::new(String::from("06"), Coordinate::new( -80.0,   20.0)),
        City::new(String::from("07"), Coordinate::new( -70.0,   30.0)),
        City::new(String::from("08"), Coordinate::new( -60.0,   40.0)),
        City::new(String::from("09"), Coordinate::new( -50.0,   50.0)),
        City::new(String::from("10"), Coordinate::new( -40.0,   60.0)),
        City::new(String::from("11"), Coordinate::new( -30.0,   70.0)),
        City::new(String::from("12"), Coordinate::new( -20.0,   80.0)),
        City::new(String::from("13"), Coordinate::new( -10.0,   90.0)),
        City::new(String::from("14"), Coordinate::new(  10.0,   90.0)),
        City::new(String::from("15"), Coordinate::new(  20.0,   80.0)),
        City::new(String::from("16"), Coordinate::new(  30.0,   70.0)),
        City::new(String::from("17"), Coordinate::new(  40.0,   60.0)),
        City::new(String::from("18"), Coordinate::new(  50.0,   50.0)),
        City::new(String::from("19"), Coordinate::new(  60.0,   40.0)),
        City::new(String::from("20"), Coordinate::new(  70.0,   30.0)),
        City::new(String::from("21"), Coordinate::new(  80.0,   20.0)),
        City::new(String::from("22"), Coordinate::new(  90.0,   10.0)),
        City::new(String::from("23"), Coordinate::new(  90.0,  -10.0)),
        City::new(String::from("24"), Coordinate::new(  80.0,  -20.0)),
        City::new(String::from("25"), Coordinate::new(  70.0,  -30.0)),
        City::new(String::from("26"), Coordinate::new(  60.0,  -40.0)),
        City::new(String::from("27"), Coordinate::new(  50.0,  -50.0)),
        City::new(String::from("28"), Coordinate::new(  40.0,  -60.0)),
        City::new(String::from("29"), Coordinate::new(  30.0,  -70.0)),
        City::new(String::from("30"), Coordinate::new(  20.0,  -80.0)),
        City::new(String::from("31"), Coordinate::new(  10.0,  -90.0)),
        City::new(String::from("32"), Coordinate::new( -10.0,  -90.0)),
        City::new(String::from("33"), Coordinate::new( -20.0,  -80.0)),
        City::new(String::from("34"), Coordinate::new( -30.0,  -70.0)),
        City::new(String::from("35"), Coordinate::new( -40.0,  -60.0)),
        City::new(String::from("36"), Coordinate::new( -50.0,  -50.0)),
        City::new(String::from("37"), Coordinate::new( -60.0,  -40.0)),
        City::new(String::from("38"), Coordinate::new( -70.0,  -30.0)),
        City::new(String::from("39"), Coordinate::new( -80.0,  -20.0)),
        City::new(String::from("40"), Coordinate::new( -90.0,  -10.0))
    ];

    let epochs = 200;
    let population_size = 500;
    let mutation_rate = 0.02;
    let elitism_size = 100;
    let print_progress = false;
    let seed = 491638; // Seed that gets the best results
    
    let mut population = Population::new(cities, population_size, seed);
    population.simulate(epochs, elitism_size, mutation_rate, print_progress);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("GA 100", |b| b.iter(|| ga_bench()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
