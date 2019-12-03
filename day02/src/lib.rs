#[cfg(test)]
mod tests {

    #[test]
    fn ex1 () {
        assert_eq!(crate::day02::run_program(vec![1,9,10,3,2,3,11,0,99,30,40,50], 0), [3500,9,10,70,2,3,11,0,99,30,40,50]);
    }

    #[test]
    fn ex2 () {
        assert_eq!(crate::day02::run_program(vec![1,0,0,0,99], 0), [2,0,0,0,99]);
    }

    #[test]
    fn ex3 () {
        assert_eq!(crate::day02::run_program(vec![2,3,0,3,99], 0), [2,3,0,6,99]);
    }

    #[test]
    fn ex4 () {
        assert_eq!(crate::day02::run_program(vec![2,4,4,5,99,0], 0), [2,4,4,5,99,9801]);
    }

    #[test]
    fn ex5 () {
        assert_eq!(crate::day02::run_program(vec![1,1,1,4,99,5,6,0,99], 0), [30,1,1,4,2,5,6,0,99]);
    }
}

pub mod day02 {
    pub fn run_program(mut v: Vec<usize>, index: usize) -> Vec<usize> {
        if v[index] != 99 {
            let new_index = v[index + 3];
            
            if v[index] == 1 {
                v[new_index] = v[v[index + 1]] + v[v[index + 2]];
            } else {
                v[new_index] = v[v[index + 1]] * v[v[index + 2]];
            }
            return run_program(v, index + 4);
        } else {
            return v;
        }
    }
}