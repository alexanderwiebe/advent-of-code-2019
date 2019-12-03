/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
extern crate day02;

use std::fs;
use std::time::{Instant};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let now = Instant::now();

    let input: String  = fs::read_to_string("day02/input.data")?.parse()?;
    let v: Vec<usize> = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    let w = day02::day02::run_program(v, 0);

    println!("{}", w[0]);
    println!("took {} us", now.elapsed().as_micros());
    Ok(())
}