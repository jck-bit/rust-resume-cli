use serde_json::{Result, value};
use colored::Colorize;

pub fn show_experience(json_data: &str) -> Result<()> {
    let v:value::Value = serde_json::from_str(json_data)?;

    let mut i: usize = 0;

    loop{
        let experience: &value::Value = &v["Experiences"][i];
        if experience.is_null(){
            break;
        }

        println!("");
        println!("{} #{}","Experience".bold() , (i+1).to_string().bold());
        println!("{} : {}", "Position".yellow().bold() , experience["Position"].as_str().unwrap().bright_green());
        println!("{} : {}" , "Company".yellow().bold(),  experience["Company"].as_str().unwrap().bright_green());
        println!("{} : {}" , "Location".yellow().bold(), experience["Location"].as_str().unwrap().bright_green());
        println!("{} : {}" , "Duration".yellow().bold(), experience["Duration"].as_str().unwrap().bright_green());
        println!("{} :", "Description".yellow().bold());

        let mut j =0;
        loop{
            let description = &experience["Description"][j];
            if description.is_null(){
                break;
            }

            println!("- {}" ,description.as_str().unwrap().bright_green());
            j += 1;
        }
        i += 1;
    }
    Ok(())
}