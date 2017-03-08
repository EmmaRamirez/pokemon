mod lib;

extern crate pokemon;
extern crate rustc_serialize;
extern crate csv;

// use std::fs::File;
// use rustc_serialize::json::decode;
use lib::pokemon::{Pokemon, get_pokemon, get_all};

fn main() {

    println!("{:?}", get_pokemon());

}
