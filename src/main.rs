use clap::Parser;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);

    match result {
        Ok(content) => {
            println!("File content:");
            println!("{}", content);
        }
        Err(error) => {
            eprintln!("Oh noes: {}", error);
        }
    }

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
