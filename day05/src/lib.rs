#[cfg(test)]
mod tests {

    use super::day05::*;

    #[test]
    fn ex1 () {
        assert_eq!(run_program(vec![1,9,10,3,2,3,11,0,99,30,40,50], 0), [3500,9,10,70,2,3,11,0,99,30,40,50]);
    }

    #[test]
    fn ex2 () {
        assert_eq!(run_program(vec![1,0,0,0,99], 0), [2,0,0,0,99]);
    }

    #[test]
    fn ex3 () {
        assert_eq!(run_program(vec![2,3,0,3,99], 0), [2,3,0,6,99]);
    }

    #[test]
    fn ex4 () {
        assert_eq!(run_program(vec![2,4,4,5,99,0], 0), [2,4,4,5,99,9801]);
    }

    #[test]
    fn ex5 () {
        assert_eq!(run_program(vec![1,1,1,4,99,5,6,0,99], 0), [30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn get_op_easy () {
        assert_eq!(get_opcode(1), 1);
        assert_eq!(get_opcode(10), 10);
        assert_eq!(get_opcode(99), 99);
    }

    #[test]
    fn get_op_param_easy () {
        assert_eq!(get_opcode_params(1), vec!["0","0","0","0"]);
        assert_eq!(get_opcode_params(99), vec!["0","0","0","0"]);
        assert_eq!(get_opcode_params(111199), vec!["1","1","1","1"]);
        assert_eq!(get_opcode_params(010199), vec!["1","0","1","0"]);
        assert_eq!(get_opcode_params(001199), vec!["1","1","0","0"]);
    }

    #[test]
    fn get_op_medium () {
        let vector = vec!["0","1","2","3"];
        let mut fdsa = asdf(vector);
        fdsa = "5";
        assert_eq!(vector, vec!["0","1","2","3"]);
    }

    #[test]
    fn get_op_param_medium () {

    }

    #[test]
    fn get_op_hard () {

    }

    #[test]
    fn get_op_param_hard () {

    }
}

pub mod day05 {
    pub fn asdf(fdsa: Vec<&str>) -> &str {
        return fdsa.get(0).unwrap();
    }
    pub fn get_opcode(command: usize) -> usize {
        return command.to_string().chars().rev().take(2).collect::<String>().chars().rev().collect::<String>().parse().unwrap();
    }

    pub fn get_opcode_params(command: usize) -> Vec<String> {
        return format!("{:0>4}", command/100).chars().rev().map(|x| format!("{}", x))/*.parse::<usize>().unwrap())*/.collect()
    }

    pub fn get_operator(inst: Vec<usize>, index: usize, mode: String) -> usize {
        match mode.as_ref() {
            "0" => return inst[inst[index]],
            "1" => return inst[index],
            _ => return 0
        }
    }
    // based on day02
    pub fn run_program(mut v: Vec<usize>, index: usize) -> Vec<usize> {
        let opcode_params = get_opcode_params(v[index]);
        match get_opcode(v[index]) { // opcode
            1 => {
                // let a = get_operator(v, index+1, opcode_params[1]);
                // let b = get_operator(v, index+2, opcode_params[2]);
                let new_index = v[index + 3];
                v[new_index] = v[v[index + 1]] + v[v[index + 2]];
                return run_program(v, index + 4);
            },
            2 => {
                let new_index = v[index + 3];
                v[new_index] = v[v[index + 1]] * v[v[index + 2]];
                return run_program(v, index + 4);
            },
            3 => { // input read int value
                // todo read value
                let input_value = 0;
                let input_value_index = v[index + 1];
                v[input_value_index] = input_value;
                return run_program(v, index + 2);
            },
            4 => { // output print to screen

                return run_program(v, index + 2);
            },
            99 | _ => return v
        }
    }
}