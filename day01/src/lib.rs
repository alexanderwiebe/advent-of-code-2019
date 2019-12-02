#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn ex1 () {
        assert_eq!(crate::day01::mass_to_fuel(12), 2);
    }

    #[test]
    fn ex2 () {
        assert_eq!(crate::day01::mass_to_fuel_rec(14, 0), 2);
    }
    #[test]
    fn ex3 () {
        assert_eq!(crate::day01::mass_to_fuel_rec(1969, 0), 966);
    }
    #[test]
    fn ex4 () {
        assert_eq!(crate::day01::mass_to_fuel_rec(100756, 0), 50346);
    }
}

pub mod day01 {
    
    pub fn mass_to_fuel(mass: u32) -> u32 {
        (mass / 3) - 2
    }

    pub fn mass_to_fuel_rec(mass: u32, total_fuel: u32) -> u32 {
        let fuel = mass_to_fuel(mass);
        if fuel >= 9 { return mass_to_fuel_rec(fuel, total_fuel + fuel) } else { return total_fuel + fuel }
    }

}