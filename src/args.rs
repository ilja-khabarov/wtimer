use clap::{Arg, App, SubCommand, ArgMatches};

pub fn parse() -> ArgMatches<'static> {
    App::new("Wtimer")
        .version("0.0.0")
        .author("Ilja Khabarov <ilja.khabarov@gmail.com>")
        .about("A console app to manage your time")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("PATH")
            .help("Set certain config file")
        )
        .get_matches()
}
