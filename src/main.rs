extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate simplesvg;

use std::fs::File;
use std::io::prelude::*;

mod command;
mod cursor;
mod direction;
mod point;

use command::Command;
use cursor::Cursor;

fn main() {
    let matches = clap::App::new("Logo")
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
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let log_level = match matches.occurrences_of("verbose") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    simplelog::TermLogger::init(log_level, simplelog::Config::default()).unwrap();

    debug!("input: {}", input);
    debug!("verbosity: {:?}", log_level);
    if log_level >= log::LevelFilter::Debug {
        debug!("We are very verbose!");
    }

    let mut f = File::open(input).expect("error opening file");

    let mut code = String::new();
    f.read_to_string(&mut code).expect("error reading file");

    debug!("Code: \n{}", code);

    info!("Running program...");
    let mut drawing = Vec::new();
    let w = 640.;
    let h = 640.;
    drawing.push(
        simplesvg::Fig::Rect(0., 0., w, h)
            .styled(simplesvg::Attr::default().fill(simplesvg::Color(0xff, 0xee, 0xee))),
    );

    let mut cursor = Cursor::new(w / 2., h / 2.);

    debug!("Start cursor: {:?}", cursor);
    for (i, line) in code.lines().enumerate() {
        debug!("read: {}", line);
        let command = parse_line(i, line);
        debug!("Command: {:?}", command);
        if let Some(c) = command {
            if let Some(f) = cursor.apply_command(&c) {
                drawing.push(f);
            }
        }
    }
    let draw_fig = simplesvg::Fig::Multiple(drawing);
    let svg_data = simplesvg::Svg(vec![draw_fig], w as u32, h as u32);
    println!("{}", svg_data);
}

fn parse_line(i: usize, line: &str) -> Option<Command> {
    match line.chars().next() {
        Some('#') | None => return None,
        Some(_) => {}
    }

    macro_rules! lpanic {
        ($($arg:tt)*) => (panic!("{} \nOn line {}: `{}`\n", format_args!($($arg)*),i,line));
    }

    match Command::from_line(line) {
        Ok(c) => return Some(c),
        Err(e) => lpanic!("error parsing command: {}", e),
    };
}
