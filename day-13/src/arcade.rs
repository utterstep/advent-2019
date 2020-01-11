use std::{
    cmp::Ordering,
    convert::TryFrom,
    io::{self, prelude::*},
    iter::once,
    thread, time,
};

use itertools::iproduct;
use serde::Deserialize;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use intcode::{IntcodeVmError, Interpreter, InterpreterState};

use crate::{
    consts::*,
    tile::{Tile, TileParseError},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmulatorMode {
    Manual,
    Auto,
    AutoVisualized,
}

#[derive(Debug)]
pub struct Emulator {
    interpreter: Interpreter,
    tiles: Vec<Tile>,
    score: Option<i64>,
    ball_x: Option<usize>,
    paddle_x: Option<usize>,
    mode: EmulatorMode,
}

#[derive(Debug)]
pub enum EmulatorError {
    IntcodeVmError(IntcodeVmError),
    TileParseError(TileParseError),
    InsufficientFiguresOnField,
}

impl From<IntcodeVmError> for EmulatorError {
    fn from(e: IntcodeVmError) -> Self {
        Self::IntcodeVmError(e)
    }
}

impl From<TileParseError> for EmulatorError {
    fn from(e: TileParseError) -> Self {
        Self::TileParseError(e)
    }
}

impl Emulator {
    pub fn new(interpreter: Interpreter, mode: EmulatorMode) -> Self {
        let tiles = iproduct!(MIN_Y..=MAX_Y, MIN_X..=MAX_X)
            .map(|(x, y)| Tile::try_from(&[x as i64, y as i64, 0][..]).unwrap())
            .collect();

        Self {
            interpreter,
            tiles,
            score: None,
            ball_x: None,
            paddle_x: None,
            mode,
        }
    }

    pub fn play(mut self) -> Result<Option<i64>, EmulatorError> {
        loop {
            match self.interpreter.get_state() {
                InterpreterState::Failed(e) => break Err(EmulatorError::IntcodeVmError(e)),
                InterpreterState::Halted => {
                    self.collect_output()?;

                    break Ok(self.score);
                }
                InterpreterState::Initial => self.interpreter.run(),
                InterpreterState::WaitingForInput => {
                    self.collect_output()?;

                    match self.mode {
                        EmulatorMode::Manual => self.display(),
                        EmulatorMode::AutoVisualized => {
                            let sleep_time = time::Duration::from_secs_f64(1. / 60.);

                            thread::sleep(sleep_time);
                            self.display();
                        }
                        _ => {}
                    }

                    let input = match self.mode {
                        EmulatorMode::Manual => match self.get_input() {
                            Some(input) => input,
                            None => continue,
                        },
                        EmulatorMode::Auto | EmulatorMode::AutoVisualized => {
                            match (self.ball_x, self.paddle_x) {
                                (Some(ball_x), Some(paddle_x)) => match ball_x.cmp(&paddle_x) {
                                    Ordering::Less => -1,
                                    Ordering::Equal => 0,
                                    Ordering::Greater => 1,
                                },
                                _ => return Err(EmulatorError::InsufficientFiguresOnField),
                            }
                        }
                    };

                    self.interpreter.run_with_input(once(&input))
                }
            }
        }
    }

    fn collect_output(&mut self) -> Result<(), EmulatorError> {
        let output = self.interpreter.get_output()?;

        for chunk in output.chunks_exact(TILE_CHUNK_SIZE) {
            let tile = Tile::try_from(chunk)?;

            if let Some((x, y)) = tile.get_coords() {
                if let Tile::Ball { x, y: _y } = tile {
                    self.ball_x = Some(x);
                }
                if let Tile::Paddle { x, y: _y } = tile {
                    self.paddle_x = Some(x);
                }

                self.tiles[y * X_SPAN + x] = tile;
            } else if let Tile::SegmentDisplay { score } = tile {
                self.score.replace(score);
            }
        }

        drop(self.interpreter.drain_output().unwrap());

        Ok(())
    }

    fn display(&self) {
        let field = self
            .tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let chr = tile.to_char();
                let (x, y) = (i % X_SPAN, i / X_SPAN);

                format!("{1}{0}", chr, if x == 0 && y > 0 { "\n" } else { "" })
            })
            .collect::<String>();

        let mut writer = io::stdout();
        writeln!(writer, "{}\n{}", termion::clear::All, field).unwrap();

        if let Some(score) = self.score {
            writeln!(writer, "Current score: {}", score).unwrap();
        }
    }

    fn get_input(&mut self) -> Option<i64> {
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        write!(stdout, "\nMake your move (a, s, d): ").unwrap();
        stdout.flush().unwrap();
        let input = stdin.keys().next()?.ok()?;

        match input {
            Key::Char('a') => Some(-1),
            Key::Char('s') => Some(0),
            Key::Char('d') => Some(1),
            _ => None,
        }
    }
}
