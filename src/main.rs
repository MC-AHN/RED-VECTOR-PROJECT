use std::fs::File;
use std::io::{self, Write};

fn main() ->  io::Result<()> {
    println!("=== RED VECTORE GENERATOR v0.1 ===");
    println!("To be Fast and Efesien");

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
    let content = format!(
        "SUPABASE_URL={}\nSUPABASE_KEY={}\n",
        url, key
    );

    //
    let mut file = File::create(".env")?;
    file.write_all(content.as_bytes())?;

    println!("Successfully, file .env are created");

    Ok(())
}
