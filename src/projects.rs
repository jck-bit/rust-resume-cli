use serde_json::{Result, value};
use colored::Colorize;


pub fn show_projects(json_data: &str) -> Result<()> {
    let v: value::Value = serde_json::from_str(json_data)?;

    let mut i: usize = 0;

    loop {
        let projects: &value::Value = &v["Projects"][i];
        if projects.is_null() {
            break;
        }

        println!("");
        println!("{} #{}", "Project".bold().bright_yellow(), (i + 1).to_string().bold());
        println!("{} : {}", "Name".bright_yellow().bold(), projects["Name"].as_str().unwrap().bright_green());
        println!("{} : {}", "Year".bright_yellow().bold(), projects["Year"].as_str().unwrap().bright_green());
        println!("{} : {}", "Stack".bright_yellow().bold(), projects["Stack"].as_str().unwrap().bright_green());
        println!("{}", "Description".to_string().bold());

        let mut j =0;
        loop{
            let description = &projects["Description"][j];
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
