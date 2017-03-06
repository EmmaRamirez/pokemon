extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use std::io::copy;
use std::io::stdout;

// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
#[cfg(test)]
mod pokemon {

    #[derive(RustcDecodable, RustcEncodable)]
    pub struct Pokemon (Vec<String>);
    const LANGUAGES:[&'static str] = &["de", "en", "fr", "ja", "ko", "ru"];


    pub fn get_pokemon() {
        let mut file = File::open("data/en.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json: Pokemon = json::decode(&data).unwrap();

    }


    #[test]
    fn get_all() {

    }

    #[test]
    fn get_random() {

    }

    #[test]
    fn get_name(id: i32, lang: LANGUAGES) {

    }

    #[test]
    fn get_id(name: str, lang: LANGUAGES) {

    }
}
