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

    #[test]
    fn three_false() {
        assert_eq!(crate::day04::three_adj_numb("123456"), false)
    }

    #[test]
    fn three_true_double() {
        assert_eq!(crate::day04::three_adj_numb("112233"), true)
    }
    #[test]
    fn three_true_triple() {
        assert_eq!(crate::day04::three_adj_numb("111466"), true)
    }
    #[test]
    fn three_true_start_triple() {
        assert_eq!(crate::day04::three_adj_numb("114666"), true)
    }
    #[test]
    fn three_false_triple() {
        assert_eq!(crate::day04::three_adj_numb("111666"), false)
    }

    #[test]
    fn six_false_triple() {
        assert_eq!(crate::day04::three_adj_numb("111111"), false)
    }

    #[test]
    fn true_then_same() {
        assert_eq!(crate::day04::three_adj_numb("112111"), true)
    }
    
    #[test]
    fn true_then_same_rev() {
        assert_eq!(crate::day04::three_adj_numb("111211"), true)
    }
    
}

pub mod day04 {
    pub fn password_check(password: usize) -> bool {
        let password_str = password.to_string();
        return inc_numb(&password_str) && three_adj_numb(&password_str);
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
            }
        }
        return ret_val;
    }

    pub fn three_adj_numb(password: &str) -> bool {
        let mut ret_val = false;
        let mut loop_count = 0;
        let mut iterator = password.chars().peekable();
        loop {
            let letter = iterator.next();
            let next = iterator.peek();
            if next == None { 
                if loop_count == 1 {
                    ret_val = true;
                }
                break;
            }
            if letter.unwrap() == *next.unwrap() {
                loop_count += 1;
                if loop_count > 1 { ret_val = false }
            } else {
                if loop_count == 1 {
                    ret_val = true;
                    break;
                }
                loop_count = 0;
            }
        }
        return ret_val;
    }

    // pub fn three_adj_numb(password: &str) -> bool {
    //     let mut ret_val = false;
    //     let mut iterator = password.chars().peekable();
    //     let mut letter = iterator.next();
    //     loop {
    //         let next = iterator.next();
    //         if next == None { break }
    //         if letter.unwrap() == next.unwrap() && (iterator.peek() == None || (iterator.peek() != None && next.unwrap() != *iterator.peek().unwrap())) {
    //             ret_val = true;
    //             break;
    //         } else {
    //             letter = iterator.next();
    //             if letter == None { break }
    //         }
    //     }
    //     return ret_val;
    // }

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