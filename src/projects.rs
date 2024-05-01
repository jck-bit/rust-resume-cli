use serde_json::{Result, value};
use colored::Colorize;
use viuer::{print_from_file, Config};

pub fn show_projects(json_data: &str) -> Result<()>{
    let v:value::Value = serde_json::from_str(json_data)?;

    let mut i:usize = 0;

    loop{
        let projects:&value::Value = &v["Projects"][i];
        if projects.is_null(){
            break;
        }

        println!("");
        println!("{} #{}", "Project".bold().bright_yellow(), (i+1).to_string().bold());
        println!("{} : {}","Name".bright_yellow().bold(), projects["Name"].as_str().unwrap().bright_green());
        println!("{} : {}", "Year".bright_yellow().bold(), projects["Year"].as_str().unwrap().bright_green());
        println!("{} : {}", "Stack".bright_yellow().bold(), projects["Stack"].as_str().unwrap().bright_green());
        println!("{}", projects["Description"].to_string().bold());
        
        println!("");
        
        //now we show the image of the project on the cli
        let mut conf = Config {
            //set offset

            x:20,
            y:4,
            width:Some(80),
            height:some(25),
            ..Default::default()

            print_from_file("img.jpg", &conf).expect("Image printing failed.");
        };
               

        i += 1;
    }

    Ok(())
}