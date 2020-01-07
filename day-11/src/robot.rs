use fnv::FnvHashMap as HashMap;

use crate::utils::minmax;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 0,
    White = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

const LEFT_TURN: i64 = 0;
const RIGHT_TURN: i64 = 1;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Robot {
    position: Position,
    orientation: Orientation,
    panels: HashMap<Position, Color>,
}

impl Robot {
    pub fn new(start_color: Color) -> Self {
        let start = Position { x: 0, y: 0 };
        let mut panels = HashMap::default();
        panels.insert(start, start_color);

        Self {
            position: start,
            orientation: Orientation::Up,
            panels,
        }
    }

    pub fn current_color(&self) -> Color {
        self.panels
            .get(&self.position)
            .cloned()
            .unwrap_or(Color::Black)
    }

    pub fn painted_panels_count(&self) -> usize {
        self.panels.len()
    }

    pub fn painted_panels_display(&self) -> String {
        let (x_min, x_max) =
            minmax(self.panels.keys().map(|p| p.x)).expect("panels non empty since creation");
        let (y_min, y_max) =
            minmax(self.panels.keys().map(|p| p.y)).expect("panels non empty since creation");
        let x_span = x_max - x_min + 1;
        let y_span = y_max - y_min + 1;

        let cols = vec![' '; x_span as usize];
        let mut rows = (0..y_span).map(|_| cols.clone()).collect::<Vec<_>>();

        for (position, &color) in &self.panels {
            if color == Color::White {
                rows[(y_max - position.y) as usize][(x_max - position.x) as usize] = '*';
            }
        }

        let lines = rows.iter().map(|row| row.iter().rev().collect::<String>());

        lines.collect::<Vec<_>>().join("\n")
    }

    pub fn process_instruction(&mut self, color: i64, rotation: i64) {
        debug_assert!(color == 0 || color == 1);

        let color = match color {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };

        self.panels.insert(self.position, color);
        self.move_rotate(rotation);
    }

    fn move_rotate(&mut self, rotation: i64) {
        debug_assert!(rotation == LEFT_TURN || rotation == RIGHT_TURN);

        if rotation == LEFT_TURN {
            match self.orientation {
                Orientation::Up => {
                    self.orientation = Orientation::Left;
                    self.position = Position {
                        x: self.position.x - 1,
                        y: self.position.y,
                    };
                }
                Orientation::Right => {
                    self.orientation = Orientation::Up;
                    self.position = Position {
                        x: self.position.x,
                        y: self.position.y + 1,
                    };
                }
                Orientation::Down => {
                    self.orientation = Orientation::Right;
                    self.position = Position {
                        x: self.position.x + 1,
                        y: self.position.y,
                    };
                }
                Orientation::Left => {
                    self.orientation = Orientation::Down;
                    self.position = Position {
                        x: self.position.x,
                        y: self.position.y - 1,
                    };
                }
            }
        } else {
            match self.orientation {
                Orientation::Up => {
                    self.orientation = Orientation::Right;
                    self.position = Position {
                        x: self.position.x + 1,
                        y: self.position.y,
                    };
                }
                Orientation::Right => {
                    self.orientation = Orientation::Down;
                    self.position = Position {
                        x: self.position.x,
                        y: self.position.y - 1,
                    };
                }
                Orientation::Down => {
                    self.orientation = Orientation::Left;
                    self.position = Position {
                        x: self.position.x - 1,
                        y: self.position.y,
                    };
                }
                Orientation::Left => {
                    self.orientation = Orientation::Up;
                    self.position = Position {
                        x: self.position.x,
                        y: self.position.y + 1,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot() {
        let mut robot = Robot::new(Color::Black);

        robot.process_instruction(1, 0);
        robot.process_instruction(0, 0);
        robot.process_instruction(1, 0);
        robot.process_instruction(1, 0);
        robot.process_instruction(0, 1);
        robot.process_instruction(1, 0);
        robot.process_instruction(1, 0);

        assert_eq!(robot.position, Position { x: 0, y: 1 });
        assert_eq!(robot.orientation, Orientation::Left);
        assert_eq!(robot.painted_panels_count(), 6);
    }
}
