/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
extern crate day01;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input: String  = fs::read_to_string("day01/input.data")?.parse()?;
    let v: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();

    let w:u32 = v.iter().map(|x| day01::day01::mass_to_fuel(*x)).sum();

    println!("{}", w);
    
    Ok(())
}