use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    println!("=== RED VECTORE GENERATOR v0.2 ===");
    println!("To be Fast and Efesien");

    //
    println!("Name Project: ");
    io::stdout().flush()?;
    let mut name_project = String::new();
    io::stdin().read_line(&mut name_project)?;
    let name_project = name_project.trim();

    //
    let status = Command::new("cargo")
        .arg("new")
        .arg(name_project)
        .status();

    match status {
        Ok(s) if s.success() => {
            // input data
            println!("URL Supabase: ");
            io::stdout().flush()?;
            let mut url = String::new();
            io::stdin().read_line(&mut url)?;

            // input supabase key
            print!("Supabase Key: ");
            io::stdout().flush()?;
            let mut key = String::new();
            io::stdin().read_line(&mut key)?;

            // delete space in url
            let url = url.trim();
            let key = key.trim();

            //
            let content = format!("SUPABASE_URL={}\nSUPABASE_KEY={}\n", url, key);

            //
            let path_env = format!("{}/.env", name_project);
            std::fs::write(path_env, content)?;

            println!("Successfully, file .env are created");
        }
        _ => {
            println!("[ERROR] Failed to create project");
        }
    }

    Ok(())
}
