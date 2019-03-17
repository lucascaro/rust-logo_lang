use std::i64;
use std::str;

pub static VALID_COMMANDS: &str =
    "FORWARD, F, LEFT, L, RIGHT , R, PENUP, U, PENDOW, D, REPEAT, R, SETCOLOR, C";

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
        Command::from_chunks(&mut line.split_whitespace())
    }

    pub fn from_chunks(chunks: &mut str::SplitWhitespace) -> Result<Command, String> {
        trace!(
            "Command from chunks: {:?}",
            chunks.clone().collect::<Vec<&str>>()
        );
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
        if s.len() == 8 {
            String::from(s)
        } else if s.len() == 3 {
            format!("0x{}", (&s[2..]).repeat(8))
        } else if s.len() != 5 {
            panic!(
                "hex color must have 1, 3 or 6 digits after 0x, , found {}: `{}`.",
                s.len(),
                s
            )
        } else {
            let chars: Vec<_> = (&s[2..]).split("").collect();
            trace!("chars for hex: `{:?}`", chars);
            let long = format!(
                "0x{}",
                chars.iter().map(|c| c.repeat(2)).collect::<String>()
            );
            trace!("normalized color from hex: `{}`", long);
            long
        }
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

    fn next_int_chunk(chunks: &mut str::SplitWhitespace) -> Result<u8, String> {
        chunks
            .next()
            .ok_or_else(|| String::from("missing color segment"))
            .and_then(|c| c.parse::<u8>().map_err(|e| format!("parsing color {}", e)))
    }

    pub fn setcolor_from_chunks(chunks: &mut str::SplitWhitespace) -> Result<Command, String> {
        let first_color_str = chunks
            .next()
            .ok_or_else(|| String::from("error parsing color: no color to parse"))?;
        if let Ok(first_color) = first_color_str.parse::<u8>() {
            let segments = vec![
                Ok(first_color),
                Command::next_int_chunk(chunks),
                Command::next_int_chunk(chunks),
            ]
            .into_iter()
            .collect::<Result<Vec<u8>, String>>();
            segments.map(|s| [s[0], s[1], s[2]])
        } else {
            Ok(Command::colors_from_hex(&first_color_str))
        }
        .map(|colors| Command::SetColor(colors[0], colors[1], colors[2]))
        .map_err(|e| format!("error parsing color components: {}", e))
    }

    pub fn repeat_from_split(chunks: &mut str::SplitWhitespace) -> Result<Command, String> {
        // chunks.collect::<Vec<&str>>().join(" ");
        let n = match chunks.next() {
            Some(text) => text
                .parse()
                .ok()
                .ok_or_else(|| format!("REPEAT: expected number, found `{}`", text))?,
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
        Ok(Command::Repeat(n, commands))
    }
}
