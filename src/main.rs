use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    println!("=== RED VECTORE GENERATOR v0.3 ===");
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
            print!("URL Supabase: ");
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

            println!("Connection to Red Vector Cloud...");

            Command::new("git")
                .arg("init")
                .current_dir(name_project)
                .status()?;

            // add to stage index
            Command::new("git")
                .arg("add")
                .arg(".")
                .current_dir(name_project)
                .status()?;

            // commit
            Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("Initial commit by Red Vector Engine")
                .current_dir(name_project)
                .status()?;

            // info
            println!("[INFO] Code Alredy send to repository");

            let repo_url = format!("git@github.com:MC-AHN/RED-VECTOR-PROJECT.git");
            Command::new("git")
                .arg("remote")
                .arg("add")
                .arg("origin")
                .arg(&repo_url)
                .current_dir(name_project)
                .status()?;

            Command::new("git")
                .arg("push")
                .arg("-u")
                .arg("origin")
                .arg("master")
                .current_dir(name_project)
                .status()?;

            println!("[INFO] Code for '{}' successfully sent to repository", name_project);
        }
        _ => {
            println!("[ERROR] Failed to create project");
        }
    }

    Ok(())
}
