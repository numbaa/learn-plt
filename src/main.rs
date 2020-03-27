use std::env;

fn usage() {
    println!("Usage: freestyle <source_file>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return;
    }
    let filename: &String = &args[1];
    println!("Executing {}...", filename);
    match freestyle::run(filename) {
        Ok(_) => println!("ok"),
        Err(msg) => println!("{}", msg),
    }
}
