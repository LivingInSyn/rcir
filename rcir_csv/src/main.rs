extern crate csv;
extern crate clap;
extern crate rcir;

use clap::{App, Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use rcir::*;

fn main() {
    let matches = App::new("plist")
        .version("1.0")
        .author("Jeremy Mill - jeremy.mill@gmail.com")
        .about("Runs a ranked choice instant runoff election from the given csv")
        .arg(Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .required(true)
            .help("The file containing the votes")
            .index(1))
        //parse matches
        .get_matches();
    // no error checking here, will panic on bad or missing values
    let f = File::open(matches.value_of("input_file").unwrap()).unwrap();
    let reader = BufReader::new(f);
    // inefficient, but should get the point across
    let mut voters = Vec::new();
    for line in reader.lines() {
        let mut votes = Vec::new();
        if let Ok(line) = line {
            for name in line.split(',') {
                votes.push(name.to_string());
            }
        }        
        voters.push(votes);
    }
    let winner = run_election(&voters);
    match winner {
        Ok(rcir::ElectionResult::Winner(w)) => {
            println!("The winner was: {}", w);
        }
        // You should also handle ties and errors here
        _ => {println!("An error or tie occured")}
    }
}
