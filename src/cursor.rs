use simplesvg::{Attr, Color, ColorAttr, Fig};

use command::Command;
use direction::{Direction, TurningDirection};
use point::Point;

#[derive(Debug, Clone)]
pub struct Cursor {
    pos: Point<f32>,
    dir: Direction,
    writing: bool,
    stroke_width: f32,
    stroke_color: ColorAttr,
}

impl Cursor {
    pub fn new(x: f32, y: f32) -> Cursor {
        Cursor {
            pos: Point { x, y },
            dir: Direction::EAST,
            writing: true,
            stroke_width: 2.,
            stroke_color: Color(0, 0, 0),
        }
    }

    pub fn rotate(&mut self, d: &TurningDirection) {
        self.dir = self.dir.rotate(d);
    }

    pub fn move_forwards(&mut self, distance: u32) -> Option<Fig> {
        let old_position = self.pos.clone();
        let dir = self.dir.value();
        self.pos = Point {
            x: self.pos.x + (distance as i32 * dir.x) as f32,
            y: self.pos.y + (distance as i32 * dir.y) as f32,
        };
        return match self.writing {
            true => Some(self.draw_line(&old_position, &self.pos)),
            false => None,
        };
    }

    fn draw_line(&self, p1: &Point<f32>, p2: &Point<f32>) -> Fig {
        debug!("line from {:?} to {:?}", p1, p2);
        return Fig::Line(p1.x, p1.y, p2.x, p2.y).styled(
            Attr::default()
                .stroke(self.stroke_color)
                .stroke_width(self.stroke_width),
        );
    }

    pub fn stop_writing(&mut self) {
        self.writing = false;
    }

    pub fn start_writing(&mut self) {
        self.writing = true;
    }

    pub fn set_size(&mut self, d: f32) {
        self.stroke_width = d;
    }
    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.stroke_color = Color(r, g, b);
    }

    pub fn apply_command(&mut self, cmd: &Command) -> Option<Fig> {
        match cmd {
            &Command::Forward(d) => return self.move_forwards(d),
            &Command::Repeat(n, ref c) => return self.repeat(n, c),
            &Command::SetSize(d) => self.set_size(d),
            &Command::SetColor(r, g, b) => self.set_color(r, g, b),
            &Command::Left => self.rotate(&TurningDirection::LEFT),
            &Command::Right => self.rotate(&TurningDirection::RIGHT),
            &Command::Up => self.stop_writing(),
            &Command::Down => self.start_writing(),
            &Command::EndRepeat => panic!("EndRepeat found in wrong context"),
        };
        debug!("Cursor: {:?}", self);
        return None;
    }

    pub fn repeat(&mut self, n: u32, cs: &Vec<Command>) -> Option<Fig> {
        let mut figs = Vec::new();
        for iteration in 0..n {
            trace!("iteration {} of {:?}", iteration, cs);
            for cmd in cs {
                if let Some(fig) = self.apply_command(cmd) {
                    figs.push(fig);
                }
            }
        }

        return match figs.len() {
            0 => None,
            _ => Some(Fig::Multiple(figs)),
        };
    }
}
