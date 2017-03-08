// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
extern crate rustc_serialize;
extern crate csv;

pub mod pokemon {
    use std::io::prelude::*;
    use std::fs::File;
    use std::path::Path;
    // use rustc_serialize::json;
    use csv;

    pub struct Row { }

    #[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
    pub struct Pokemon {
        species: i32,
        language: i32,
        name: String,
        genus: Option<String>,
    }

    #[allow(dead_code, unused_variables, unused_mut)]
    pub fn get_pokemon() -> Vec<Pokemon> {

        fn search<P: AsRef<Path>>(file_path: P) -> Vec<Pokemon> {
            let mut found:Vec<Pokemon> = vec![];
            let file = File::open(file_path).unwrap();
            let mut rdr = csv::Reader::from_reader(file).has_headers(true);

            let rows:Vec<Pokemon> = rdr.decode().collect::<csv::Result<Vec<Pokemon>>>().unwrap();

            rows
        }

        search("data/en.csv")
    }

    #[allow(dead_code)]
    pub fn get_all() {

    }

    #[allow(dead_code)]
    pub fn get_random() {
        // let pokes = get_pokemon();
        // pokes.0
    }

    #[allow(dead_code, unused_variables)]
    pub fn get_name(id: i32) -> Pokemon {
        let mut poke = &mut *get_pokemon().clone();
        poke[0]
    }

}
