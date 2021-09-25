// Copyright (c) 2021 Lu Neder
// Licensed under the Open Software License version 3.0

// import needed crates
extern crate config;
extern crate glob;
use config::*; //required for config.toml, account.toml and other .toml files
use std::env; //required for grabbing command line args
use std::fs; //required to move and rename fles (fs::rename), to copy files (fs::copy), to create directories (fs::create_dir_all) and to create files (fs::File)
use symlink::*; //required to create symbolic links
use std::path::Path; // required for file paths
use std::collections::HashMap; // required for .toml files
use glob::glob; // required to get .toml files
use std::io; // a bunch of stuff
use std::io::prelude::*; //create text files
use text_io::scan; //ask for input
use bcrypt::{DEFAULT_COST, hash, verify}; // encrypt password
use rpassword; // safe password input


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
    } else if option == "add-account" {
        add_account()
    }
    println!("WORK IN PROGRESS");
}


// Print Help
fn sam_help(ver: String) {
    println!("Samantha {}", ver);
    checkifinstalled();
    println!("IMPORTANT - All file paths must be absolute. Do not use relative paths (like ./ or ../), they will not work.");
    println!("IMPORTANT - While not required, we recommend you to NOT add spaces to the filenames or to the directory names");
    println!("IMPORTANT - Run Samantha as root/administrator or make sure your user has read and write permissions on your Samantha root");
    println!("--help: Show this help list");
    println!("--init [/PATH/TO/Config.toml]: Configure Samantha for the first time, following the Config.toml file with the options you chose.");
    println!("--create-account")
}


//checks if Samantha is installed by seeing if /samantha_root exists
fn checkifinstalled() {
    let installed = Path::new("/samantha_root").exists();
    if installed == true {
        let samantha_root = fs::read_to_string("/samantha_root");
        println!("Samantha is installed and her root is at {:?}", samantha_root.unwrap());
    } else {
        println!("Samantha is not installed");
    }
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
            .map(|path| config::File::from(path.unwrap()))
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
    rootindicator(samantha_root.to_string()).expect("error creating file at / indicating Samantha root")//Runs the Function that creates a text file at / telling Sam where her root is

}


//Function that creates the Samantha root directory
fn rootdir(samantha_root: String) -> std::io::Result<()> {
    println!("Samantha root: {}", samantha_root);
    fs::create_dir_all(samantha_root)?;
    Ok(())
}


//Function that copies Config.toml to Samantha root
fn moveconfig(configpath: String, newconfigpath: String) -> std::io::Result<()> {
    println!("Config: {}", newconfigpath);
    fs::copy(configpath, newconfigpath)?;
    Ok(())
}


//Function that creates a text file at / telling Sam where her root is
fn rootindicator(samantha_root: String) -> std::io::Result<()> {
    let mut samrootindicator = fs::File::create("/samantha_root")?;
    samrootindicator.write_all(samantha_root.as_bytes())?;
    Ok(())
}


// Create new account
fn add_account() {
    //Get the stuff in Config.toml
    let samantha_root = fs::read_to_string("/samantha_root").unwrap();
    let mut settings = Config::default();
    let configpath = samantha_root.to_string() + &"/Config.toml".to_string();
    settings
        .merge(glob(&configpath)
            .unwrap()
            .map(|path| config::File::from(path.unwrap()))
            .collect::<Vec<_>>())
        .unwrap();
        // Save the Config.toml as a config variable as a HashMap
    let mut config =
        settings
        .try_into::<HashMap<String, String>>()
        .unwrap();
    // println!("{:#?}", config); //prints the config
    //check config file for each available field and if set as True ask for the info
    let name: String = if config["ask_name"] == "True" { //start setting the name variable
        let name: String;
        println!("Name: ");
        scan!("{}", name); //ask for Name input and saves as a name variable
        println!("{}", name);
        name } else {
        let no: String = "".to_string(); //saves an empty string as a no variable that'll be the name variable if config tells not to ask for it
        no }; //end of name
    let username: String = if config["ask_username"] == "True" { //start setting the username variable
        let username: String;
        println!("@userame: ");
        scan!("{}", username); //ask for username input
        let username: String = if username.starts_with("@") {username} else { //checks if an @ is at the start of the username
            let username: String = "@".to_string() + &username; username }; //adds an @ to the start of the username if the user didn't do it yet
            println!("{}", username);
            username} else {
                let no: String = "".to_string(); //keeps username empty if config tells not to ask for it
                no }; // end of username
    let email: String = if config["ask_email"] == "True" { //start setting the email variable
        let email: String;
        println!("E-mail: ");
        scan!("{}", email);//ask for input
        if email.contains("@") {} else {println!("NOT AN E-MAIL");//check if has an @, do nothing if it does
        std::process::exit(1);}//close Sam with code 1 if doesnt have an @ after printing NOT AN EMAIL
        println!("{}", email);
        email } else {
            let no: String = "".to_string(); //leave email empty if config tells not to ask for it
            no }; //end of email
    let phone: String = if config["ask_phone"] == "True" { //start setting the phone variable
        let phone: String;
        println!("Phone: ");
        scan!("{}", phone); //ask for phone input and saves as a phone variable
        println!("{}", phone);
        phone } else {
            let no: String = "".to_string(); //leave empty if config tells not to ask for it
            no }; //end of phone
    let pronouns: String = if config["ask_pronouns"] == "True" { //start setting the pronouns variable
        let pronouns: String;
        println!("Pronouns: ");
        scan!("{}", pronouns); //ask for pronouns input and saves as a pronouns variable
        println!("{}", pronouns);
        pronouns } else {
            let no: String = "".to_string(); //leave empty if config tells not to ask for it
            no }; //end of pronouns
    let gender: String = if config["ask_gender"] == "True" { //start setting the gender variable
        let gender: String;
        println!("Gender: ");
        scan!("{}", gender); //ask for gender input and saves as a gender variable
        println!("{}", gender);
        gender } else {
            let no: String = "".to_string(); //leave empty if config tells not to ask for it
            no }; //end of gender
    let birthday: String = if config["ask_birthday"] == "True" { //start setting the birthday variable
        let birthday: String;
        println!("Birthday: ");
        scan!("{}", birthday); //ask for birthday input and saves as a birthday variable
        println!("{}", birthday);
        birthday } else {
            let no: String = "".to_string(); //leave empty if config tells not to ask for it
            no }; //end of birthday
    let password: String;
    scan!("{}", password);
    let password: String = hash(password, DEFAULT_COST).unwrap();

    println!("{}, {}, {}, {}, {}, {}, {}, {}", name, username, email, phone, pronouns, gender, birthday, password);


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
