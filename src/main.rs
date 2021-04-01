// Copyright (c) 2021 Lu Neder
// Licensed under the Open Software License version 3.0

// import needed crates
extern crate config;
extern crate glob;
use config::*; //required for config.toml, account.toml and other .toml files
use std::env; //required for grabbing command line args
use std::fs; //required to move and rename fles (fs::rename), to copy files (fs::copy) and to create directories (fs::create_dir_all)
use symlink::*; //required to create symbolic links
use std::path::Path; // required for file paths
use std::collections::HashMap; // required for .toml files
use glob::glob; // required to get .toml files
use std::io; // a bunch of stuff


fn main() {
    let ver: String = "1.0.0".to_string(); //sets ver vaiable to current Samantha version

    // Get command args and save it as an args variable
    let args: Vec<String> = env::args().collect();
    // run the option the user chose (--help, --init, etc)
    let option = &args[1];
    if option == "help" {
        sam_help(ver);
    } else if option == "init" {
        init();
    }
    println!("WORK IN PROGRESS");
}


// Print Help
fn sam_help(ver: String) {
    println!("Samantha {}", ver);
    println!("IMPORTANT - All file paths must be absolute. Do not use relative paths (like ./ or ../), they will not work.");
    println!("IMPORTANT - While not required, we recommend you to NOT add spaces to the filenames or to the directory names");
    println!("IMPORTANT - Run Samantha as root/administrator or make sure your user has read and write permissions on your Samantha root");
    println!("--help: Show this help list");
    println!("--init [/PATH/TO/Config.toml]: Configure Samantha for the first time, following the Config.toml file with the options you chose.");
    println!("--create-account")
}


// First usage stuff
fn init() {
    let args: Vec<String> = env::args().collect(); //Get command line args
    let configpath = &args[2]; // Get the 2nd arg (Config.toml path)
    //Get the stuff in Config.toml
    let mut settings = Config::default();
    settings
        .merge(glob(configpath)
            .unwrap()
            .map(|path| File::from(path.unwrap()))
            .collect::<Vec<_>>())
        .unwrap();
// Save the Config.toml as a config variable as a HashMap
    let mut config =
        settings
        .try_into::<HashMap<String, String>>()
        .unwrap();
    println!("{:#?}", config); //prints the config
    let samantha_root = &config["samantha_root"]; //get the samantha_root configuration and saves it as a samantha_root variable
    let newconfigpath = samantha_root.to_owned() + "/Config.toml"; // path where Config.toml will be copied to
    rootdir(samantha_root.to_string()).expect("error creating Samantha root"); //runs the Function that creates the Samantha root directory
    moveconfig(configpath.to_string(), newconfigpath.to_string()).expect("error copying Config.toml");//Runs the Function that copies Config.toml to Samantha root

}
//Function that creates the Samantha root directory
fn rootdir(samantha_root: String) -> std::io::Result<()> {
    println!("Samantha root: {}", samantha_root);
    fs::create_dir_all(samantha_root)?;
    Ok(())
}
fn moveconfig(configpath: String, newconfigpath: String) -> std::io::Result<()> {
    println!("Config: {}", newconfigpath);
    fs::copy(configpath, newconfigpath)?;
    Ok(())
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
fn post() {

}
