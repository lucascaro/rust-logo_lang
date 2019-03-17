extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate simplesvg;

use std::fs::File;
use std::io::prelude::*;

mod cli;
mod command;
mod cursor;
mod direction;
mod logging;
mod point;

use command::Command;
use cursor::Cursor;

fn get_code_from_file(input: &str) -> String {
    let mut f = File::open(input).expect("error opening file");
    let mut code = String::new();
    f.read_to_string(&mut code).expect("error reading file");
    code
}

fn create_drawing(code: &str) -> simplesvg::Svg {
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
    simplesvg::Svg(vec![draw_fig], w as u32, h as u32)
}

fn parse_line(i: usize, line: &str) -> Option<Command> {
    match line.chars().next() {
        Some('#') | None => return None,
        Some(_) => {}
    }

    macro_rules! lpanic {
        ($($arg:tt)*) => (panic!("{} \nOn line {}: `{}`\n", format_args!($($arg)*),i,line));
    }

    Command::from_line(line)
        .map_err(|e| lpanic!("error parsing command: {}", e))
        .ok()
}

fn main() {
    let matches = cli::args();

    let input = matches.value_of("input").unwrap();
    logging::init(matches.occurrences_of("verbose"));

    debug!("input: {}", input);
    let code = get_code_from_file(input);
    debug!("Code: \n{}", code);

    info!("Running program...");
    let svg_data = create_drawing(&code);
    println!("{}", svg_data);
}
