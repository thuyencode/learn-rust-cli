use std::env::args;
use std::path::PathBuf;

struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let pattern = args().nth(1).expect("no pattern given");
    let path = args().nth(2).expect("no path given");

    let args = Cli {
        pattern,
        path: PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
