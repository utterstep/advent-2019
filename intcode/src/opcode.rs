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
        if value < 100 {
            let parameter_modes = [ParameterMode::Position; 3];

            let operation = match value {
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

            return Self { parameter_modes, operation };
        }

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

        value /= 100;

        // my eyes :( but this unrollment gives like +20% perf on day-9 (4ms vs 5ms on turbo-locked Core i7-8750H)
        let parameter_modes = match value {
            000 => [ParameterMode::Position, ParameterMode::Position, ParameterMode::Position],
            001 => [ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Position],
            002 => [ParameterMode::Relative, ParameterMode::Position, ParameterMode::Position],
            010 => [ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position],
            011 => [ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Position],
            012 => [ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Position],
            020 => [ParameterMode::Position, ParameterMode::Relative, ParameterMode::Position],
            021 => [ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Position],
            022 => [ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Position],
            100 => [ParameterMode::Position, ParameterMode::Position, ParameterMode::Immediate],
            101 => [ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Immediate],
            102 => [ParameterMode::Relative, ParameterMode::Position, ParameterMode::Immediate],
            110 => [ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Immediate],
            111 => [ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Immediate],
            112 => [ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Immediate],
            120 => [ParameterMode::Position, ParameterMode::Relative, ParameterMode::Immediate],
            121 => [ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Immediate],
            122 => [ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Immediate],
            200 => [ParameterMode::Position, ParameterMode::Position, ParameterMode::Relative],
            201 => [ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Relative],
            202 => [ParameterMode::Relative, ParameterMode::Position, ParameterMode::Relative],
            210 => [ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Relative],
            211 => [ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Relative],
            212 => [ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Relative],
            220 => [ParameterMode::Position, ParameterMode::Relative, ParameterMode::Relative],
            221 => [ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Relative],
            222 => [ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Relative],
            _ => return Self {
                operation: Operation::Unknown,
                parameter_modes: [ParameterMode::Position; 3],
            },
        };

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
