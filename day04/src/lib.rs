#[cfg(test)]
mod tests {

    #[test]
    fn two_true() {
        assert_eq!(crate::day04::two_adj_numb("123455"), true)
    }

    #[test]
    fn two_false() {
        assert_eq!(crate::day04::two_adj_numb("123456"), false)
    }

    #[test]
    fn inc_true() {
        assert_eq!(crate::day04::inc_numb("123456"), true)
    }

    #[test]
    fn inc_false() {
        assert_eq!(crate::day04::inc_numb("123436"), false)
    }
}

pub mod day04 {
    pub fn password_check(password: usize) -> bool {
        let password_str = password.to_string();
        return two_adj_numb(&password_str) && inc_numb(&password_str);
    }

    pub fn two_adj_numb(password: &str) -> bool {
        let mut ret_val = false;
        let mut iterator = password.chars().peekable();
        loop {
            let letter = iterator.next();
            let next = iterator.peek();
            if next == None { break }
            if letter.unwrap() == *next.unwrap() {
                ret_val = true;
                break;
            }

        }
        return ret_val;
    }

    pub fn inc_numb(password: &str) -> bool {
        let mut ret_val = true;
        let mut iterator = password.chars().peekable();
        loop {
            let letter = iterator.next().unwrap().to_digit(10).unwrap();
            let next = iterator.peek();
            if next == None { break }
            let next_value = next.unwrap().to_digit(10).unwrap();
            if letter > next_value {
                ret_val = false;
                break;
            }

        }
        return ret_val;
    }
}