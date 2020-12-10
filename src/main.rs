use wtimer::args;

fn main() {
    let matches = args::parse();
    if !matches.is_present("config") {
        println!("'config' parameter does not present");
    }
    println!("Hello, world!");
}
