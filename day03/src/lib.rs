#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test1 () {
        let points = crate::day03::take_steps("D2", crate::day03::Point { x: 0, y: 0 });
        
        assert_eq!(points[0].x, 0 );
        assert_eq!(points[0].y, -1 );
        assert_eq!(points[1].x, 0 );
        assert_eq!(points[1].y, -2 );
    }

    
    #[test]
    fn test2 () {
        let points = crate::day03::take_steps("R2", crate::day03::Point { x: 0, y: 0 });
        
        assert_eq!(points[0].x, 1 );
        assert_eq!(points[0].y, 0 );
        assert_eq!(points[1].x, 2 );
        assert_eq!(points[1].y, 0 );
    }

    fn test3 () {
        let mut points1 = crate::day03::take_steps("R2", crate::day03::Point { x: 0, y: 0 });
        points1.extend(crate::day03::take_steps("U2", crate::day03::Point { x: 2, y: 0 }));
        let mut points2 = crate::day03::take_steps("U2", crate::day03::Point { x: 0, y: 0 });
        points2.extend(crate::day03::take_steps("R2", crate::day03::Point { x: 0, y: 2 }));
        let intersections = crate::day03::find_intersection(points1, points2);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].x, 2);
        assert_eq!(intersections[0].y, 2);
        
    }

    fn find_nearest_point() {
        
    }
}

pub mod day03 {
    
    #[derive(PartialEq, PartialOrd, Copy, Clone)]
    pub struct Point {
        pub x: isize,
        pub y: isize,
        pub steps: isize
    }

    pub fn take_steps(instruction: &str, origin: Point) -> Vec<Point> {
        let mut iter = instruction.chars();
        let direction = iter.next().unwrap();
        let steps: usize = iter.collect::<String>().parse().unwrap();

        let mut points: Vec<Point> = Vec::new();

        if direction == 'D' {
            for step in 1..(steps + 1) {
                points.push(Point { x: origin.x, y: origin.y - step as isize, steps: step as isize + origin.steps });
            }
        } else if direction == 'U' {
            for step in 1..(steps + 1) {
                points.push(Point { x: origin.x, y: origin.y + step as isize, steps: step as isize + origin.steps });
            }
        } else if direction == 'R' {
            for step in 1..(steps + 1) {
                points.push(Point { x: origin.x + step as isize, y: origin.y, steps: step as isize + origin.steps });
            }
        } else if direction == 'L' {
            for step in 1..(steps + 1) {
                points.push(Point { x: origin.x - step as isize, y: origin.y, steps: step as isize + origin.steps });
            }
        }

        return points;
    }

    pub fn find_intersection(wire1: Vec<Point>, wire2: Vec<Point>) -> isize {
        // let mut points: Vec<(Point, Point)> = Vec::new();
        let mut lowest_steps = isize::max_value();
        for w1 in wire1.iter() {
            for w2 in wire2.iter() {
                if w1.x == w2.x && w1.y == w2.y && (w1.y != 0 && w2.y != 0 && w1.x != 0 && w2.x != 0) {
                    // points.push((*w1,*w2));
                    if w1.steps + w2.steps < lowest_steps {
                        lowest_steps = w1.steps + w2.steps;
                    }
                }
            }
        }
        return lowest_steps;
        // return points;
    }

    pub fn find_shortest_taxi(points: Vec<Point>) -> Point {
        let mut point_iter = points.iter();
        let mut max = point_iter.next().unwrap();
        for p in point_iter {
            if p.x.abs() + p.y.abs() < max.x.abs() + max.y.abs() {
                max = p;
            }
        }
        return *max;
    }
}