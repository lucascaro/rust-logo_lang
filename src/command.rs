use std::i64;
use std::str;

pub static VALID_COMMANDS: &str = "FORWARD, F, LEFT, L, RIGHT , R, PENUP, U, PENDOW, D";

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Left,
    Right,
    Up,
    Down,
    SetSize(f32),
    SetColor(u8, u8, u8),
    Repeat(u32, Vec<Command>),
    EndRepeat,
}

impl Command {
    pub fn from_line(line: &str) -> Result<Command, String> {
        return Command::from_chunks(&mut line.split_whitespace());
    }

    pub fn from_chunks(chunks: &mut str::SplitWhitespace) -> Result<Command, String> {
        match chunks.next() {
            Some("PENUP") | Some("U") => Ok(Command::Up),
            Some("PENDOWN") | Some("D") => Ok(Command::Down),
            Some("LEFT") | Some("L") => Ok(Command::Left),
            Some("RIGHT") | Some("R") => Ok(Command::Right),
            Some("SETSIZE") | Some("S") => Command::setsize_from_str(chunks.next()),
            Some("SETCOLOR") | Some("C") => Command::setcolor_from_chunks(chunks),
            Some("FORWARD") | Some("F") => Command::forward_from_str(chunks.next()),
            Some("REPEAT") => Command::repeat_from_split(chunks),
            Some("]") => Ok(Command::EndRepeat),
            Some(c) => Err(format!(
                "invalid command: '{}'. Expected one of: {}",
                c, VALID_COMMANDS
            )),
            None => Err(String::from("empty command")),
        }
    }

    pub fn forward_from_str(distance: Option<&str>) -> Result<Command, String> {
        match distance {
            Some(text) => match text.parse() {
                Ok(n) => Ok(Command::Forward(n)),
                Err(e) => Err(format!(
                    "unable to parse distance: {}. Expected a number.",
                    e
                )),
            },
            None => Err(format!(
                "move command is missing parameter. Expected: MOVE [distance], found: `{:?}`.",
                distance
            )),
        }
    }
    pub fn setsize_from_str(distance: Option<&str>) -> Result<Command, String> {
        match distance {
            Some(text) => match text.parse() {
                Ok(n) => Ok(Command::SetSize(n)),
                Err(e) => Err(format!(
                    "unable to parse distance: {}. Expected a number.",
                    e
                )),
            },
            None => Err(format!(
                "move command is missing parameter. Expected: MOVE [distance], found: `{:?}`.",
                distance
            )),
        }
    }

    fn normalize_hex_str(s: &str) -> String {
        if &s[..2] != "0x" {
            panic!("single argument colors must start with `0x`, got `{}`", s);
        }
        let mut long = String::from(s);
        if s.len() < 8 {
            let chars: Vec<_> = match s.len() {
                5 => (&s[2..]).split("").collect(),
                _ => panic!(
                    "hex color must have 3 or 5 digits after 0x, , found `{}`.",
                    s
                ),
            };
            trace!("chars for hex: `{:?}`", chars);
            long = format!(
                "0x{}{}{}{}{}{}",
                chars[1], chars[1], chars[2], chars[2], chars[3], chars[3]
            );
            trace!("normalized color from hex: `{}`", long);
        }
        return long;
    }

    fn colors_from_hex(s: &str) -> [u8; 3] {
        debug!("get colors from hex: {}", s);
        let hex = Command::normalize_hex_str(s);
        if let Ok(x) = i64::from_str_radix(&hex[2..], 16) {
            let b1: u8 = ((x >> 16) & 0xff) as u8;
            let b2: u8 = ((x >> 8) & 0xff) as u8;
            let b3: u8 = ((x) & 0xff) as u8;
            trace!("get colors from hex: {}, {}, {}", b1, b2, b3);
            return [b1, b2, b3];
        }
        panic!("error converting hex to color: {}", s);
    }

    pub fn setcolor_from_chunks(chunks: &mut str::SplitWhitespace) -> Result<Command, String> {
        let mut colors: [u8; 3] = [0, 0, 0];
        if chunks.clone().count() == 1 {
            colors = Command::colors_from_hex(chunks.next().unwrap());
        } else {
            for i in 0..3 {
                match chunks.next() {
                    Some(text) => match text.parse() {
                        Ok(n) => colors[i] = n,
                        Err(e) => return Err(format!("error parsing color component: {}", e)),
                    },
                    None => return Err(format!("Setcolor requires three numeric parameters.",)),
                }
            }
        }
        return Ok(Command::SetColor(colors[0], colors[1], colors[2]));
    }

    pub fn repeat_from_split(chunks: &mut str::SplitWhitespace) -> Result<Command, String> {
        // chunks.collect::<Vec<&str>>().join(" ");
        let n = match chunks.next() {
            Some(text) => text.parse()
                .ok()
                .ok_or(format!("REPEAT: expected number, found `{}`", text))?,
            None => return Err(String::from("REPEAT: expected number, found nothing.")),
        };
        match chunks.next() {
            Some("[") => { /*Ignore expected start of the loop*/ }
            Some(s) => return Err(format!("REPEAT: expected [, found `{}`.", s)),
            None => return Err(String::from("REPEAT: expected [, found nothing.")),
        }
        // Read commands from the loop until a ] is found
        let mut commands: Vec<Command> = Vec::new();

        loop {
            match Command::from_chunks(chunks) {
                Ok(Command::EndRepeat) => break,
                Ok(c) => commands.push(c),
                Err(e) => return Err(e),
            };
        }
        return Ok(Command::Repeat(n, commands));
    }
}
