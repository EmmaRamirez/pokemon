mod lib;

extern crate pokemon;
extern crate rustc_serialize;
extern crate csv;
extern crate rand;

// use std::fs::File;
// use rustc_serialize::json::decode;
use lib::pokemon::{Pokemon, get_pokemon, get_random, get_all};

fn main() {

    println!("{:?}", get_all());
    println!("{:?}", get_pokemon(1, 9));
    println!("{:?}", get_random());

}
