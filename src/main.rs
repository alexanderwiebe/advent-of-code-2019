/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
extern crate day04;

use std::time::{Instant};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let now = Instant::now();
    let mut password_count = 0;
    for password in 206938..679128 {
        if day04::day04::password_check(password) {
            password_count += 1;
        }
    }

    println!("total passwords is {}", password_count);
    println!("took {} us", now.elapsed().as_micros());
    Ok(())
}