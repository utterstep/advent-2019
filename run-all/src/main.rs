use std::{env::var, error::Error, str::FromStr};

use advent_utils::{read_file, Solver};

fn run<S: Solver + FromStr<Err = Box<dyn Error>>>() -> Result<(), Box<dyn Error>> {
    let input_file = format!(
        "{}/day-{}/input.txt",
        var("BASE_PATH").unwrap_or_else(|_| ".".to_owned()),
        S::day_number()
    );
    let input_data = read_file(input_file)?;
    let solver: S = input_data.parse()?;

    for part in S::implemented_parts() {
        println!("{}", solver.solve(part));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run::<day_1::Solution>()?;
    run::<day_2::Solution>()?;
    run::<day_3::Solution>()?;
    run::<day_4::Solution>()?;
    run::<day_5::Solution>()?;
    run::<day_6::Solution>()?;
    run::<day_7::Solution>()?;
    run::<day_8::Solution>()?;
    run::<day_9::Solution>()?;
    run::<day_10::Solution>()?;
    run::<day_11::Solution>()?;
    run::<day_12::Solution>()?;
    run::<day_13::Solution>()?;
    run::<day_14::Solution>()?;
    run::<day_17::Solution>()?;
    run::<day_19::Solution>()?;
    run::<day_22::Solution>()?;
    run::<day_23::Solution>()?;
    run::<day_24::Solution>()?;

    Ok(())
}
