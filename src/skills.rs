use colored::Colorize;
use serde_json::{Result, Value};

pub fn show_skills(json_data: &str) ->Result<()>{
    let data: Value = serde_json::from_str(json_data)?;

    println!("");
    println!("{}: {}","Languages".bold(),  data["Languages"].as_str().unwrap().bright_green());
    println!("{}: {}", "Web Technologies".bold(), data["Web Technologies"].as_str().unwrap().bright_green());
    println!("{}: {}", "Databases".bold(), data["Databases"].as_str().unwrap().bright_green());
    println!("{}: {}", "DevOps".bold(), data["DevOps"].as_str().unwrap().bright_green());

    Ok(())
}