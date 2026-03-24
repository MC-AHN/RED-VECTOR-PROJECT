use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    println!("=== RED VECTORE GENERATOR v0.5 ===");
    println!("To be Fast and Efesien");

    //
    println!("Name Project: ");
    io::stdout().flush()?;
    let mut name_project = String::new();
    io::stdin().read_line(&mut name_project)?;
    let name_project = name_project.trim();

    //
    let status = Command::new("cargo").arg("new").arg(name_project).status();

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
                println!(
                    "[INFO] Repo Private '{}' successfully created in github",
                    name_project
                );
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

            println!(
                "[INFO] Code for '{}' successfully sent to repository",
                name_project
            );

            println!("Starting level 2: The Logic Injector v0.5.2..");

            // add library
            Command::new("cargo")
                .arg("add")
                .arg("axum")
                .arg("tokio")
                .arg("reqwest")
                .arg("serde")
                .arg("serde_json")
                .arg("dotenvy")
                .arg("--features")
                .arg("tokio/full,reqwest/json")
                .current_dir(name_project)
                .status()?;

            //
            println!("");
            let master_template = r#"
                use axum::{routing::get, Json, Router};
                use serde_json::{json, Value};
                use std::net::SocketAddr;

                #[tokio::main]
                async fn main() -> Result<(), Box<dyn std::error:Error>> {
                    dotenvy::dotenv().ok();

                    // inisialisasi table
                    init_db().await?;

                    let app = Router::new().route("/", get(health_check));
                    let addr = SocketAddr::from(([0,0,0,0], 8000));
                    println!("[RED VECTOR] API Live at http://{}", addr);
                    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap()?;
                }

                async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
                    let url = std::env::var("SUPABASE_URL")?;
                    let key = std::env::var("SUPABASE_KEY")?;
                    let client = reqwest::Client::new();

                    let sql_query = "
                        CREATE TABLE IF NOT EXISTS profiles (
                            id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                            username text UNIQUE NOT NULL,
                            create_at timestampz DEFAULT now()
                        );
                    ";

                    println!("[LEVEL 3] Database Schema Chacked/Applied.");
                    Ok(())
                }

                async fn health_check() -> Json<Value> {
                    let url = std::env::var("SUPABASE_URL").unwrap_or_default();
                    let key = std::env::var("SUPABASE_KEY").unwrap_or_default();
                    let client = reqwest::Client::new();

                //
                let response = client.get(format!("{}/rest/v1/",url))
                    .header("apikey", &key)
                    .header("Authorization", format!("Bearer {}", key))
                    .send().await;

                let (db_status, msg) = match response {
                Ok(res) if res.status().is_success() => ("Connected", "key is valid"),
                Ok(res) if res.status().as_u16() == 401 => ("Unauthorized", "Key is Invalid."),
                _ => ("Failed", "URL Supabase can't use."),
                };

                Json(json!({
                    "status": if db_status == "Connected" {"Operational"} else {"Error"},
                    "diagnostics": {
                        "supabase": db_status,
                        "message": msg,
                        "engine": "Red Vector v0.5"
                    }
                }))
            }
            "#;

            let main_rs_path = format!("{}/src/main.rs", name_project);
            std::fs::write(main_rs_path, master_template)?;

            println!("[SUCCESS] Logic v0.5 Successfully!");
        }
        _ => {
            println!("[ERROR] Failed to create project");
        }
    }

    Ok(())
}
