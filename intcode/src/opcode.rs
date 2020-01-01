#[derive(Debug)]
pub(crate) enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjRelBase,
    End,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
pub(crate) struct Opcode {
    pub operation: Operation,
    pub parameter_modes: [ParameterMode; 3],
}

impl From<i64> for Opcode {
    fn from(mut value: i64) -> Self {
        let operation = match value % 100 {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpIfTrue,
            6 => Operation::JumpIfFalse,
            7 => Operation::LessThan,
            8 => Operation::Equals,
            9 => Operation::AdjRelBase,
            99 => Operation::End,
            _ => Operation::Unknown,
        };

        let mut parameter_modes = [ParameterMode::Position; 3];

        value /= 100;
        let mut i = 0;

        while value > 0 {
            parameter_modes[i] = match value % 10 {
                2 => ParameterMode::Relative,
                1 => ParameterMode::Immediate,
                0 => ParameterMode::Position,
                _ => {
                    return Self {
                        operation: Operation::Unknown,
                        parameter_modes,
                    };
                }
            };

            i += 1;
            value /= 10;
        }

        Self {
            operation,
            parameter_modes,
        }
    }
}

impl Opcode {
    pub(crate) fn parameters_count(&self) -> usize {
        match self.operation {
            Operation::Add | Operation::Multiply | Operation::LessThan | Operation::Equals => 3,
            Operation::JumpIfTrue | Operation::JumpIfFalse => 2,
            Operation::Input | Operation::Output | Operation::AdjRelBase => 1,
            Operation::End | Operation::Unknown => 0,
        }
    }
}
