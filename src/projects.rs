use serde_json::{Result, value};
use colored::Colorize;
use pic;

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
        println!("{}", projects["Description"].to_string().bold());
        println!("");

        // Display Image
        if let Some(image_path) = projects["image"].as_str() {
            let path1 = std::path::PathBuf::from(image_path);
            let mut options = pic::options::Options::new(vec![path1]);

            // Set your options
            options.set_position(Some(10), None);
            options.set_size(Some(50), Some(50));
            options.upscale();

            // Preview
            if let Err(err) = pic::previewer::preview(&mut std::io::stdout(), &mut options) {
                eprintln!("{err}");
            }
        }

        i += 1;
    }

    Ok(())
}
