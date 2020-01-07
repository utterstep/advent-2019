use std::iter::once;

use intcode::{IntcodeVmError, Interpreter, InterpreterState};

use crate::robot::{Color, Robot};

#[derive(Debug)]
pub enum PaintingError {
    IntcodeError(IntcodeVmError),
    InsufficientOutput,
}

pub fn paint_panels(
    mut interpreter: Interpreter,
    start_color: Color,
) -> Result<Robot, PaintingError> {
    let mut robot = Robot::new(start_color);

    loop {
        match interpreter.get_state() {
            InterpreterState::Failed(e) => break Err(PaintingError::IntcodeError(e)),
            InterpreterState::Initial => interpreter.run(),
            InterpreterState::Halted => break Ok(robot),
            InterpreterState::WaitingForInput => {
                let color = robot.current_color() as i64;
                interpreter.run_with_input(once(&color));

                let mut output = interpreter
                    .drain_output()
                    .map_err(PaintingError::IntcodeError)?;

                let (color, rotation) = (
                    output.next().ok_or(PaintingError::InsufficientOutput)?,
                    output.next().ok_or(PaintingError::InsufficientOutput)?,
                );

                robot.process_instruction(color, rotation);
            }
        }
    }
}
