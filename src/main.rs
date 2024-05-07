mod experience;
mod skills;
mod projects;
mod contacts;

use std::fs;
use colored::Colorize;
use inquire::Select;
use experience::show_experience;
use skills::show_skills;
use projects::show_projects;
use contacts::show_socials;  
// mod music;

// use music::music_player;
fn main() {
    
    //running the the music player on a separate thread to ensure it keeps playing while the program runs

    // println!("");
    // println!("");

    // std::thread::spawn(|| {
    //     music_player();
    // });


    println!("Hello there! I'm {}, a fullstack developer and am currently learning new technologies,", "Jack Kinyanjui".bold().bright_yellow());
    let options = vec!["About", "Experience",  "Skills", "Projects", "Contact", "Exit"];
    loop {
        let choice = Select::new("What would you Like to know?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0]{
                    println!("");
                    println!("I am a driven {}, I like learning new stuff and buiding applicatons."," software developer".bold().bright_yellow());
                    println!("I have knowledge in programming languages such as {}.","Rust, Python, JavaScript,  and more".bold().bright_purple());
                    println!("I have a passion to solve complex problems");
                    println!();
                }
                else if choice == options[1]{
                    let file_path = "./data/experience/experience.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("could not load the file");
                    let res = show_experience(&contents);

                    match res {
                        Ok(_res) => print!(""),
                        Err(_) => println!("Error in Experience.rs"),
                    }
                    
                }
                else if choice == options[2] {
                    let file_path = "./data/skills/skills.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("could not load the file");
                    let res = show_skills(&contents);

                    match res {
                        Ok(_res) => print!(""),
                        Err(_) => println!("Error in Skills.rs"),
                    }
                }
                else if choice == options[3] {
                    let file_path = "./data/projects/projects.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("could not load the file");
                    let res = show_projects(&contents);

                    match res {
                        Ok(_res) => print!(""),
                        Err(_) => println!("Error in Projects.rs"),
                    }

                }

                else if choice == options[4] {
                    show_socials();
                }

                else if choice == options[5]{ 
                    println!("Thank you for being here");
                    break;
                }
            },
            Err(_) => println!("Error: Could not get choice"),

        }
    }
}