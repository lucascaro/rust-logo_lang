pub static VALID_TURNING_DIRECTIONS: &str = "LEFT, L, RIGHT, R";

#[derive(Debug, PartialEq, Eq)]
pub enum TurningDirection {
    LEFT,
    RIGHT,
}

impl TurningDirection {
    pub fn from_string(s: &str) -> Result<TurningDirection, String> {
        return match s {
            "LEFT" | "L" => Ok(TurningDirection::LEFT),
            "RIGHT" | "R" => Ok(TurningDirection::RIGHT),
            _ => Err(format!(
                "invalid direction: [{}] expected one of {}",
                s, VALID_TURNING_DIRECTIONS
            )),
        };
    }
}
