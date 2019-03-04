use point::Point;

#[derive(Debug, PartialEq, Eq)]
pub enum TurningDirection {
    LEFT,
    RIGHT,
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub enum Direction {
    EAST,
    NORTH,
    WEST,
    SOUTH,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::EAST,
    Direction::NORTH,
    Direction::WEST,
    Direction::SOUTH,
];

impl Direction {
    fn _next(&self, d: i32) -> Direction {
        if let Ok(i) = DIRECTIONS.binary_search(self) {
            // Add 4 in case of dealing with -1
            let n = (i as i32) + d + 4;
            let n = (n % 4) as usize;
            debug!("_nexting with {} + {}: {}", i as i32, (i as i32) + d, n);
            return DIRECTIONS[n].clone();
        }
        panic!("error nexting direction {:?}: {}", self, d);
    }

    pub fn value(&self) -> Point<i32> {
        match *self {
            Direction::EAST => Point { x: 1, y: 0 },
            Direction::NORTH => Point { x: 0, y: -1 },
            Direction::WEST => Point { x: -1, y: 0 },
            Direction::SOUTH => Point { x: 0, y: 1 },
        }
    }

    pub fn rotate(&self, d: &TurningDirection) -> Direction {
        let delta = match d {
            &TurningDirection::LEFT => 1,
            &TurningDirection::RIGHT => -1,
        };
        trace!("Rotate {:?}: {:?}", self, self._next(delta));
        return self._next(delta);
    }
}
