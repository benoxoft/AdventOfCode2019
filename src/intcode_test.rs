
#[cfg(test)]
mod tests {
    use crate::intcode::run_intcode;

    #[test]
    fn test_intcode_basic1() {
        let mut program1 = vec![1, 0, 0, 0, 99];
        let result1 = vec![2, 0, 0, 0, 99];
        run_intcode(&mut program1, 0, 0);
        assert_eq!(result1, program1);
    }
    
    #[test]
    fn test_intcode_basic2() {
        let mut program2 = vec![2, 3, 0, 3, 99];
        let result2 = vec![2, 3, 0, 6, 99];
        run_intcode(&mut program2, 0, 0);
        assert_eq!(result2, program2);
    }

    #[test]
    fn test_intcode_basic3() {
        let mut program3 = vec![2, 4, 4, 5, 99, 0];
        let result3 = vec![2, 4, 4, 5, 99, 9801];
        run_intcode(&mut program3, 0, 0);
        assert_eq!(result3, program3);
    }

    #[test]
    fn test_intcode_basic4() {
        let mut program4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result4 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        run_intcode(&mut program4, 0, 0);
        assert_eq!(result4, program4);
    }
}

