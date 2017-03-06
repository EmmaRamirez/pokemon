// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
extern crate rustc_serialize;

mod pokemon {



    #[derive(RustcDecodable, RustcEncodable)]
    pub struct Pokemon (Vec<String>);

    #[allow(dead_code, unused_variables)]
    pub fn get_pokemon() -> Pokemon {
        use std::fs::File;
        use rustc_serialize::json::decode;

        let mut file = File::open("data/en.json").unwrap();
        let mut data = String::new();

        let json: Pokemon = decode(&data).unwrap();

        json
    }

    #[allow(dead_code)]
    fn get_all() {

    }

    #[allow(dead_code)]
    fn get_random() {

    }

    #[allow(dead_code, unused_variables)]
    fn get_name(id: i32) {

    }
}
