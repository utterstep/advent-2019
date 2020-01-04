mod opcode;
mod vm;

pub use vm::IntcodeVmError;
use vm::{IntcodeVM, IntcodeVmStopCause};

#[derive(Debug, Clone, Copy)]
pub enum InterpreterState {
    Initial,
    Halted,
    WaitingForInput,
    Failed(IntcodeVmError),
}

pub struct Interpreter {
    state: InterpreterState,
    vm: IntcodeVM,
    output: Vec<i64>,
}

impl Interpreter {
    pub fn run(&mut self) {
        self.run_with_input(std::iter::empty())
    }

    pub fn run_with_input<'a>(&mut self, input: impl IntoIterator<Item = &'a i64>) {
        match self.state {
            InterpreterState::Halted | InterpreterState::Failed(_) => {}
            InterpreterState::Initial | InterpreterState::WaitingForInput => {
                let vm = &mut self.vm;

                match vm.run_with_io(input, &mut self.output) {
                    Ok(IntcodeVmStopCause::Halted) => self.state = InterpreterState::Halted,
                    Ok(IntcodeVmStopCause::WaitingForInput) => {
                        self.state = InterpreterState::WaitingForInput
                    }
                    Err(e) => self.state = InterpreterState::Failed(e),
                }

                #[cfg(debug_assertions)]
                match self.state {
                    InterpreterState::Failed(_) | InterpreterState::Halted => {
                        println!(
                            "VM stopped: {} opcodes processed",
                            self.vm.processed_opcode_counter
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn get_state(&self) -> InterpreterState {
        self.state
    }

    pub fn get_code(&self) -> Result<&[i64], IntcodeVmError> {
        match self.state {
            InterpreterState::Failed(e) => Err(e),
            _ => Ok(self.vm.get_code()),
        }
    }

    pub fn get_output(&self) -> Result<&[i64], IntcodeVmError> {
        match self.state {
            InterpreterState::Failed(e) => Err(e),
            _ => Ok(&self.output),
        }
    }

    pub fn into_output(self) -> Result<Vec<i64>, IntcodeVmError> {
        match self.state {
            InterpreterState::Failed(e) => Err(e),
            _ => Ok(self.output),
        }
    }
}

impl From<Vec<i64>> for Interpreter {
    fn from(code: Vec<i64>) -> Self {
        Self {
            state: InterpreterState::Initial,
            output: Vec::new(),
            vm: code.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        macro_rules! test_intcode {
            ($input: expr, $expected: expr) => {
                let mut interpreter: Interpreter = $input.to_vec().into();
                interpreter.run();

                let result = interpreter.get_code().unwrap();
                assert_eq!(result[..], $expected[..]);
            };
        }

        // day 2
        test_intcode!([1, 0, 0, 0, 99], [2, 0, 0, 0, 99]);

        test_intcode!([2, 3, 0, 3, 99], [2, 3, 0, 6, 99]);

        test_intcode!([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801]);

        test_intcode!([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99]);

        // day 5
        test_intcode!([1002, 4, 3, 4, 33], [1002, 4, 3, 4, 99]);

        test_intcode!([1101, 100, -1, 4, 0], [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_intcode_io_examples() {
        macro_rules! test_io {
            ($input: expr => $code: expr, $expected: expr) => {
                let mut interpreter: Interpreter = $code.to_vec().into();
                interpreter.run_with_input(&$input);

                let result = interpreter.get_output().unwrap();
                assert_eq!(result[..], $expected[..]);
            };
        }

        // day-5

        // input equals 8 (positional)
        test_io!([1] => [3, 9, 8, 9, 10, 9, 4, 9, 99,-1, 8], [0]);
        test_io!([8] => [3, 9, 8, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([9] => [3, 9, 8, 9, 10, 9, 4, 9, 99,-1, 8], [0]);

        // input less than 8 (positional)
        test_io!([1] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([5] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([8] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);
        test_io!([9] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);

        // input equals 8 (immediate)
        test_io!([1] => [3, 3, 1108,-1, 8, 3, 4, 3, 99], [0]);
        test_io!([8] => [3, 3, 1108,-1, 8, 3, 4, 3, 99], [1]);
        test_io!([9] => [3, 3, 1108,-1, 8, 3, 4, 3, 99], [0]);

        // input less than 8 (positional)
        test_io!([1] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([5] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([8] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);
        test_io!([9] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);

        // input non-zero (positional)
        test_io!([0] => [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99,-1, 0, 1, 9], [0]);
        test_io!([1] => [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99,-1, 0, 1, 9], [1]);
        test_io!([-1] => [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99,-1, 0, 1, 9], [1]);

        // input non-zero (immediate)
        test_io!([0] => [3, 3, 1105,-1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [0]);
        test_io!([1] => [3, 3, 1105,-1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [1]);
        test_io!([-1] => [3, 3, 1105,-1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [1]);

        // cmp input to 8 (999 — less, 1000 — eq, 1001 — greater)
        test_io!([1] => [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ], [999]);
        test_io!([8] => [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ], [1000]);
        test_io!([81] => [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ], [1001]);

        // day-9

        // quine
        test_io!([] => [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
        ], [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);

        // 16-digit
        test_io!([] => [1102, 34915192, 34915192, 7, 4, 7, 99, 0], [1219070632396864]);

        // identity
        test_io!([] => [104, 1125899906842624, 99], [1125899906842624]);
    }
}
