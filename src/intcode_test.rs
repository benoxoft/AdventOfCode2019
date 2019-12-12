
#[cfg(test)]
mod tests {
    use crate::intcode::{run_intcode, ZeroInputGenerator, NullHandler, InputGenerator, OutputHandler, Program};

    #[test]
    fn test_intcode_basic1() {
        let mut program1 = vec![1, 0, 0, 0, 99];
        let result1 = vec![2, 0, 0, 0, 99];
        run_intcode(&mut program1, &ZeroInputGenerator(), &NullHandler());
        assert_eq!(result1, program1);
    }
    
    #[test]
    fn test_intcode_basic2() {
        let mut program2 = vec![2, 3, 0, 3, 99];
        let result2 = vec![2, 3, 0, 6, 99];
        run_intcode(&mut program2, &ZeroInputGenerator(), &NullHandler());
        assert_eq!(result2, program2);
    }

    #[test]
    fn test_intcode_basic3() {
        let mut program3 = vec![2, 4, 4, 5, 99, 0];
        let result3 = vec![2, 4, 4, 5, 99, 9801];
        run_intcode(&mut program3, &ZeroInputGenerator(), &NullHandler());
        assert_eq!(result3, program3);
    }

    #[test]
    fn test_intcode_basic4() {
        let mut program4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result4 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        run_intcode(&mut program4, &ZeroInputGenerator(), &NullHandler());
        assert_eq!(result4, program4);
    }

    #[test]
    fn test_prog_day5() {
        static TEST_VALUE: i64 = 32534;

        let test_ig = || -> InputGenerator {
            Box::new(|_| TEST_VALUE)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, TEST_VALUE);
            })
        };

        let mut program1 = vec![3,0,4,0,99];
        run_intcode(&mut program1, &test_ig(), &test_oh());

        let mut program2 = vec![1101,100,-1,4,0];
        run_intcode(&mut program2, &test_ig(), &NullHandler());    
    }

    #[test]
    fn test_day5_part2_1() {

        let test_ig = || -> InputGenerator {
            Box::new(|_| 8)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 1);
            })
        };
        let mut program1 = vec![3,9,8,9,10,9,4,9,99,-1,8];
        run_intcode(&mut program1, &test_ig(), &test_oh());
    }

    #[test]
    fn test_day5_part2_2() {

        let test_ig = || -> InputGenerator {
            Box::new(|_| 18)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 0);
            })
        };
        let mut program1 = vec![3,9,8,9,10,9,4,9,99,-1,8];
        run_intcode(&mut program1, &test_ig(), &test_oh());

    }

    #[test]
    fn test_day5_part2_3() {

        let test_ig = || -> InputGenerator {
            Box::new(|_| 5)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 1);
            })
        };
        let mut program2 = vec![3,9,7,9,10,9,4,9,99,-1,8];
        run_intcode(&mut program2, &test_ig(), &test_oh());
    }

    #[test]
    fn test_day5_part2_4() {
        let test_ig = || -> InputGenerator {
            Box::new(|_| 15)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 0);
            })
        };
        let mut program2 = vec![3,9,7,9,10,9,4,9,99,-1,8];
        run_intcode(&mut program2, &test_ig(), &test_oh());
    }

    #[test]
    fn test_day5_part2_5() {
        let test_ig = || -> InputGenerator {
            Box::new(|_| 8)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 1);
            })
        };
        let mut program3 = vec![3,3,1108,-1,8,3,4,3,99];
        run_intcode(&mut program3, &test_ig(), &test_oh());
    }

    #[test]
    fn test_day5_part2_6() {
        let test_ig = || -> InputGenerator {
            Box::new(|_| 18)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 0);
            })
        };
        let mut program3 = vec![3,3,1108,-1,8,3,4,3,99];
        run_intcode(&mut program3, &test_ig(), &test_oh());
    }
    
    #[test]
    fn test_day5_part2_7() {
        let test_ig = || -> InputGenerator {
            Box::new(|_| 5)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 1);
            })
        };
        let mut program4 = vec![3,3,1107,-1,8,3,4,3,99];
        run_intcode(&mut program4, &test_ig(), &test_oh());
    }

    #[test]
    fn test_day5_part2_8() {
        let test_ig = || -> InputGenerator {
            Box::new(|_| 8)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 0);
            })
        };
        let mut program4 = vec![3,3,1107,-1,8,3,4,3,99];
        run_intcode(&mut program4, &test_ig(), &test_oh());
    }

    #[test]
    fn test_jumps() {
        let test_ig = || -> InputGenerator {
            Box::new(|_| 0)
        };
        
        let test_oh = || -> OutputHandler {
            Box::new(|output: i64| {
                assert_eq!(output, 1);
            })
        };
        let mut program1 = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        run_intcode(&mut program1, &test_ig(), &test_oh());
    }
}
