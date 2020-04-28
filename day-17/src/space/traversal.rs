use super::{Object, Orientation, Space};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    TurnRight,
    TurnLeft,
    MoveForward(usize),
}

impl From<&Command> for String {
    fn from(cmd: &Command) -> String {
        match cmd {
            Command::MoveForward(steps) => steps.to_string(),
            Command::TurnLeft => "L".to_owned(),
            Command::TurnRight => "R".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct SpaceTraverse {
    robot_idx: usize,
    space: Space,
}

impl SpaceTraverse {
    fn next_idx(&self, orientation: Orientation) -> Option<usize> {
        let idx = self.robot_idx;
        let width = self.space.width;
        let (x, y) = (idx % width, idx / width);

        let next_idx = match orientation {
            Orientation::Up => idx.checked_sub(width),
            Orientation::Right => idx.checked_add(1),
            Orientation::Down => idx.checked_add(width),
            Orientation::Left => idx.checked_sub(1),
        }?;

        let (next_x, next_y) = (next_idx % width, next_idx / width);

        if (x != next_x) ^ (y != next_y) {
            Some(next_idx)
        } else {
            None
        }
    }

    fn next_object(&self, orientation: Orientation) -> Option<&Object> {
        let next_idx = self.next_idx(orientation)?;

        self.space.map.get(next_idx)
    }
}

impl Iterator for SpaceTraverse {
    type Item = Command;

    fn next(&mut self) -> Option<Command> {
        let robot = &self.space.map[self.robot_idx];
        let orientation = robot.get_orientation().expect("not a robot at robot idx");

        let mut forward_steps = 0;

        loop {
            if let Some(Object::Scaffold) | Some(Object::CleanScaffold) =
                self.next_object(orientation)
            {
                forward_steps += 1;

                let next_idx = self
                    .next_idx(orientation)
                    .expect("is Some due to if clause");

                self.space.map[next_idx] = Object::CleanScaffold;
                self.space.map.swap(self.robot_idx, next_idx);

                self.robot_idx = next_idx;

                continue;
            } else if forward_steps > 0 {
                break Some(Command::MoveForward(forward_steps));
            }

            let right_turn = orientation.right();
            if let Some(Object::Scaffold) = self.next_object(right_turn) {
                self.space.map[self.robot_idx] = Object::Robot(right_turn);

                break Some(Command::TurnRight);
            }

            let left_turn = orientation.left();
            if let Some(Object::Scaffold) = self.next_object(left_turn) {
                self.space.map[self.robot_idx] = Object::Robot(left_turn);

                break Some(Command::TurnLeft);
            }

            break None;
        }
    }
}

impl Space {
    pub fn traverse(self) -> SpaceTraverse {
        let robot_idx = self
            .map
            .iter()
            .position(|object| {
                if let Object::Robot(_) = object {
                    true
                } else {
                    false
                }
            })
            .expect("there is no robot on the map!");

        SpaceTraverse {
            robot_idx,
            space: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_traversal() {
        let space: Space = indoc!(
            "
            #######...#####
            #.....#...#...#
            #.....#...#...#
            ......#...#...#
            ......#...###.#
            ......#.....#.#
            ^########...#.#
            ......#.#...#.#
            ......#########
            ........#...#..
            ....#########..
            ....#...#......
            ....#...#......
            ....#...#......
            ....#####......"
        )
        .parse()
        .unwrap();

        let commands = space.traverse().collect::<Vec<_>>();

        assert_eq!(commands.len(), 28);

        let expected = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
            .split(',')
            .map(|s| match s {
                "R" => Command::TurnRight,
                "L" => Command::TurnLeft,
                digit => Command::MoveForward(digit.parse().unwrap()),
            })
            .collect::<Vec<_>>();

        assert_eq!(commands, expected);
    }
}
