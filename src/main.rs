//use wtimer::args;

use wtimer::control::Control;

fn main() {
    /*
    let matches = args::parse();
    if !matches.is_present("config") {
        println!("'config' parameter does not present");
    }
    */
    let c = Control::load(None, None).expect("Argh");
    c.run();

    println!("Hello, world!");
}
