#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate toml;

mod setup;
use std::path::Path;
use std::collections::HashMap;
use std::string::String;
use toml::Value;
use setup::*;

fn main() {
    let slots = vec![0.5,0.5,1.0];
    let prefs = vec![1,2,0];
    let assigns = vec![1,0,2];

    // let path = Path::new("./test.toml");
    //
    // println!("{:?}", prefs);
    // let test = Prefs::from_vec(prefs);
    // println!("{:?}", test);

    let toml_str = r#"
                    [slots.class1]
                    time = "morning"
                    level = "A2"

                    [slots.class2]
                    time = "afternoon"
                    level = "B1"

                    [preferences.name1]
                    time = {morning = 0, afternoon = 1, evening = -2}
                    level = {A1 = 1, A2 = -1, B1 = 2, B2 = 2, C1 = 3}

                    [preferences.name2]
                    time = {morning = 3, afternoon = 2, evening = 1}
                    level = {A1 = 2, A2 = 2, B1 = 1, B2 = 0, C1 = -1}
                    "#;
    let values: Value = toml_str.parse().unwrap();
    println!("{}", values);
}
