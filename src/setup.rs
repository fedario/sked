use std::path::Path;

#[derive(Debug)]
pub struct Prefs {
    prefs: Vec<f32>,
}

impl Prefs {
    pub fn from_vec(input_prefs: Vec<i8>) -> Prefs {
        let sum: f32 = input_prefs
                        .iter()
                        .map(|entry| *entry as f32)
                        .sum();
        let norm_prefs = input_prefs
                        .iter()
                        .map(|entry| *entry as f32 / sum)
                        .collect();
        Prefs {
            prefs: norm_prefs,
        }
    }
    pub fn from_file(path: &Path) -> Prefs{
        Prefs {
            prefs: Vec::new(),
        }
    }
}


pub fn score(slots: Vec<f32>, prefs: Vec<f32>, assigns: Vec<f32>) -> f32 {
    for slot in slots.iter(){
        println!("{:?}", slot);
    }
    1.0
}
