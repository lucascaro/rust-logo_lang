pub fn args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("Logo")
        .version("1.0.0")
        .author("Lucas Caro")
        .about("Simple logo interpreter in Rust")
        .arg(
            clap::Arg::with_name("input")
                .help("Input file")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches()
        .to_owned()
}
