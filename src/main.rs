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

            // 
            let gitignore_path = format!("{}/.gitignore", name_project);
            let gitignore_content = ".env\n/target\nCargo.lock";
            std::fs::write(gitignore_path, gitignore_content)?;

            println!("Connection to Red Vector Cloud...");

            Command::new("git")
                .arg("init")
                .current_dir(name_project)
                .status()?;

            Command::new("git")
                .arg("branch")
                .arg("-M")
                .arg("main")
                .current_dir(name_project)
                .status()?;

            // create repo
            let gh_status = Command::new("gh")
                .arg("repo")
                .arg("create")
                .arg(name_project)
                .arg("--private")
                .arg("--source=.")
                .arg("--remote=origin")
                .current_dir(name_project)
                .status()?;

            if gh_status.success() {
                println!("[INFO] Repo Private '{}' successfully created in github", name_project);
            }

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

            Command::new("git")
                .arg("push")
                .arg("-u")
                .arg("origin")
                .arg("main")
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
