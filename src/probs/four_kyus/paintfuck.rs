fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let cmds: Vec<char> = code.chars().collect();
    let mut data = vec![vec![false; width]; height];
    let mut data_pointer = (0, 0);
    let mut instruction_pointer = 0;
    let mut iteration = 0;

    while iteration < iterations {
        if instruction_pointer >= cmds.len() {
            break;
        }
        let cmd = cmds[instruction_pointer];
        let mut step = 1;
        match cmd {
            'n' => {
                data_pointer.0 = if data_pointer.0 == 0 {
                    height - 1
                } else {
                    data_pointer.0 - 1
                };
            }
            'e' => {
                data_pointer.1 += 1;
                if data_pointer.1 >= width {
                    data_pointer.1 = 0;
                }
            }
            's' => {
                data_pointer.0 += 1;
                if data_pointer.0 >= height {
                    data_pointer.0 = 0;
                }
            }
            'w' => {
                data_pointer.1 = if data_pointer.1 == 0 {
                    width - 1
                } else {
                    data_pointer.1 - 1
                };
            }
            '*' => {
                data[data_pointer.0][data_pointer.1] ^= true;
            }
            '[' => {
                if !data[data_pointer.0][data_pointer.1] {
                    let mut nest_count = 1;
                    for (i, &c) in cmds[instruction_pointer + 1..].iter().enumerate() {
                        match c {
                            '[' => nest_count += 1,
                            ']' => nest_count -= 1,
                            _ => (),
                        }
                        if nest_count == 0 {
                            instruction_pointer = instruction_pointer + i + 1;
                            break;
                        }
                    }
                }
            }
            ']' => {
                if data[data_pointer.0][data_pointer.1] {
                    let mut nest_count = 1;
                    for (i, &c) in cmds[..instruction_pointer].iter().enumerate().rev() {
                        match c {
                            ']' => nest_count += 1,
                            '[' => nest_count -= 1,
                            _ => (),
                        }
                        if nest_count == 0 {
                            instruction_pointer = i;
                            break;
                        }
                    }
                }
            }
            _ => step = 0,
        }
        iteration += step;
        instruction_pointer += 1;
    }

    //return our state
    data.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|i| if i { "1".to_string() } else { "0".to_string() })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\r\n")
}

#[cfg(test)]
mod paintfuck_tests {
    use super::*;

    #[test]
    fn test_nested() {
        assert_eq!(
            interpreter("*[s[e]*]", 5, 5, 5),
            "10000\r\n10000\r\n00000\r\n00000\r\n00000"
        )
    }

    #[test]
    fn simple_cases() {
        assert_eq!((&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9)), ("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
        assert_eq!((&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)), ("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
        assert_eq!((&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9)), ("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
        assert_eq!((&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), ("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
        assert_eq!((&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), ("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
    }
}
