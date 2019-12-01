/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
use std::fs;
 
 fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input: String  = fs::read_to_string("day01/input.data")?.parse()?;
    let v: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();


    for i in &v {
        println!("{}", i);
    }

    Ok(())
}