#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn ex1 () {
        assert_eq!(super::day01::mass_to_fuel(12), 2);
    }
}

mod day01 {
    
    pub fn mass_to_fuel(mass: u32) -> u32 {
        (mass / 3) - 2
    }

}