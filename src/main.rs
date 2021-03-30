// Copyright (c) 2021 Lu Neder
// Licensed under the Open Software License version 3.0

// import needed crates
use toml::value; //required for config.toml, account.toml and other .toml files
use std::env; //required for grabbing command line args



fn main() {
    let ver: String = "1.0.0".to_string(); //sets ver vaiable to current Samantha version

    // Get command args and save it as an args variable
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    if option = "help" {
        sam_help(ver)
    }
    println!("WORK IN PROGRESS");
}

fn sam_help(ver: String) {
    println!("Samantha {}", ver);
    
}
