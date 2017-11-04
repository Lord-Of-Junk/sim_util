use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;

use provider::Provider;

pub struct TraceProvider {
    numbers: Vec<f64>,
    current_number: usize,
}

impl Provider<f64> for TraceProvider {
    fn next(&mut self) -> f64 {
        if self.current_number >= self.numbers.len() {
            panic!("Insufficient Numbers to Fulfill Request! Simulation Aborted!");
        }
        let out = self.numbers[self.current_number];
        self.current_number += 1;
        out
    }
}

impl TraceProvider {
    pub fn new(filename: &String) -> TraceProvider {
        let mut out = TraceProvider {
            numbers: Vec::<f64>::new(),
            current_number: 0,
        };
        out.add_numbers(filename);
        out
    }

    pub fn add_numbers(&mut self, filename: &String) {
        
        // Create a path to the file
        let path = Path::new(filename);

        let file = match File::open(&path) {
            Err(_) => panic!("Couldn't read {}! File does not exist!", path.display()),
            Ok(file) => file,
        };

        let buffer = BufReader::new(&file);

        for line in buffer.lines() {
            let num: f64 = line.unwrap().parse().unwrap();
            self.numbers.push(num);
        }
    }

    
}

#[cfg(test)]
mod trace_provider_tests {

    use provider::trace_provider::*;

    // Test to see if we panic when we do not have enough numbers!
    #[test]
    #[should_panic]
    fn test_panic() {
        let file: String = String::from("data/too-short.dat");
        let mut p = TraceProvider::new(&file);
        loop {
            p.next();
        }
    }

    // Test to be sure the right numbers happen in the right order
    #[test]
    fn test_predictable() {
        let mut p = TraceProvider::new(&String::from("data/predictable.dat"));
        for i in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9].iter() {
            assert_eq!(*i as f64, p.next());
        }
    }
}
