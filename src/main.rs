/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
extern crate day03;

use std::fs;
use std::time::{Instant};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let now = Instant::now();

    let input: String  = fs::read_to_string("day03/input.data")?.parse()?;
    let wire1 = input.lines().nth(0);
    let wire2 = input.lines().nth(1);
    let v1: Vec<&str> = wire1.unwrap().split(",").collect::<Vec<&str>>();
    let v2: Vec<&str> =  wire2.unwrap().split(",").collect::<Vec<&str>>();

    println!("read in");

    let mut origin = day03::day03::Point { x: 0, y: 0 };
    let mut points1: Vec<day03::day03::Point> = vec![];
    for i in v1 {
        let points = day03::day03::take_steps(i, origin);
        origin = *points.last().unwrap();
        points1.extend(points);
    }
    println!("created points on v1");

    origin = day03::day03::Point { x: 0, y: 0 };
    let mut points2: Vec<day03::day03::Point> = vec![];
    for i in v2 {
        let points = day03::day03::take_steps(i, origin);
        origin = *points.last().unwrap();
        points2.extend(points);
    }
    println!("created points on v2");

    
    let intersection = day03::day03::find_intersection(points1, points2);
    println!("found intersections");

    let shortest = day03::day03::find_shortest_taxi(intersection);

    println!("shortest path is x: {}, y: {}", shortest.x, shortest.y);
    println!("total distance is: {}", shortest.x.abs() + shortest.y.abs());
    println!("took {} us", now.elapsed().as_micros());
    Ok(())
}