extern crate genetic_algorithm;

use genetic_algorithm::{city, population};

// use std::io;

/// Main function
fn main() {
    // genetic_algorithm::module::example::hello_lib();

    let cities = vec![
        city::City::new(String::from("01"), city::Coordinate::new(-100.0,  0.0)),
        city::City::new(String::from("02"), city::Coordinate::new( 100.0,  0.0)),
        city::City::new(String::from("03"), city::Coordinate::new(   0.0,  100.0)),
        city::City::new(String::from("04"), city::Coordinate::new(   0.0,  -100.0)),
        city::City::new(String::from("05"), city::Coordinate::new( -90.0,  10.0)),
        city::City::new(String::from("06"), city::Coordinate::new( -80.0,  20.0)),
        city::City::new(String::from("07"), city::Coordinate::new( -70.0,  30.0)),
        city::City::new(String::from("08"), city::Coordinate::new( -60.0,  40.0)),
        city::City::new(String::from("09"), city::Coordinate::new( -50.0,  50.0)),
        city::City::new(String::from("10"), city::Coordinate::new( -40.0,  60.0)),
        city::City::new(String::from("11"), city::Coordinate::new( -30.0,  70.0)),
        city::City::new(String::from("12"), city::Coordinate::new( -20.0,  80.0)),
        city::City::new(String::from("13"), city::Coordinate::new( -10.0,  90.0)),
        city::City::new(String::from("14"), city::Coordinate::new(  10.0,  90.0)),
        city::City::new(String::from("15"), city::Coordinate::new(  20.0,  80.0)),
        city::City::new(String::from("16"), city::Coordinate::new(  30.0,  70.0)),
        city::City::new(String::from("17"), city::Coordinate::new(  40.0,  60.0)),
        city::City::new(String::from("18"), city::Coordinate::new(  50.0,  50.0)),
        city::City::new(String::from("19"), city::Coordinate::new(  60.0,  40.0)),
        city::City::new(String::from("20"), city::Coordinate::new(  70.0,  30.0)),
        city::City::new(String::from("21"), city::Coordinate::new(  80.0,  20.0)),
        city::City::new(String::from("22"), city::Coordinate::new(  90.0,  10.0)),
        city::City::new(String::from("23"), city::Coordinate::new(  90.0, -10.0)),
        city::City::new(String::from("24"), city::Coordinate::new(  80.0, -20.0)),
        city::City::new(String::from("25"), city::Coordinate::new(  70.0, -30.0)),
        city::City::new(String::from("26"), city::Coordinate::new(  60.0, -40.0)),
        city::City::new(String::from("27"), city::Coordinate::new(  50.0, -50.0)),
        city::City::new(String::from("28"), city::Coordinate::new(  40.0, -60.0)),
        city::City::new(String::from("29"), city::Coordinate::new(  30.0, -70.0)),
        city::City::new(String::from("30"), city::Coordinate::new(  20.0, -80.0)),
        city::City::new(String::from("31"), city::Coordinate::new(  10.0, -90.0)),
        city::City::new(String::from("32"), city::Coordinate::new( -10.0, -90.0)),
        city::City::new(String::from("33"), city::Coordinate::new( -20.0, -80.0)),
        city::City::new(String::from("34"), city::Coordinate::new( -30.0, -70.0)),
        city::City::new(String::from("35"), city::Coordinate::new( -40.0, -60.0)),
        city::City::new(String::from("36"), city::Coordinate::new( -50.0, -50.0)),
        city::City::new(String::from("37"), city::Coordinate::new( -60.0, -40.0)),
        city::City::new(String::from("38"), city::Coordinate::new( -70.0, -30.0)),
        city::City::new(String::from("39"), city::Coordinate::new( -80.0, -20.0)),
        city::City::new(String::from("40"), city::Coordinate::new( -90.0, -1.00))
    ];

    let mut pop = population::Population::new();

    pop.generate_individuals(cities, 100, 1.0);

    // let mut user_input = String::new();
    // io::stdin().read_line(&mut user_input).expect("String");
}
