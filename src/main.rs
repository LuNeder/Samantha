// Copyright (c) 2021 Lu Neder
// Licensed under the Open Software License version 3.0

// import needed crates
use toml::value; //required for config.toml, account.toml and other .toml files
use std::env; //required for grabbing command line args
use std::fs::{rename, copy}; //required to move and rename fles (fs::rename) and to copy files (fs::copy)
use symlink::* //required to create symbolic links


fn main() {
    let ver: String = "1.0.0".to_string(); //sets ver vaiable to current Samantha version

    // Get command args and save it as an args variable
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    if option == "--help" {
        sam_help(ver)
    }
    println!("WORK IN PROGRESS");
}


//Help
fn sam_help(ver: String) {
    println!("Samantha {}", ver);
    println!("IMPORTANT - All file paths must be absolute. Do not use, unless it's extremely necessary, relative paths (like ./ or ../), as thet were not tested and might fail.")
    println!("--help: Show this help list")
    println!("--init [""""/PATH/TO/Config.toml""""]: Configure Samantha for the first time, following the Config.toml file with the options you chose.")

}


// First usage stuff
fn init() {

}


// Create new account
fn add_account() {

}


// Delete an account, keeping data
fn disable_account() {

}


// Completely deletes an account, deleting all its data
fn full_delete_account() {

}


//
fn
