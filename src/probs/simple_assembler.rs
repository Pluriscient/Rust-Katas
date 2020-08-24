use std::collections::HashMap;
use std::str::FromStr;

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers: HashMap<String, i64> = HashMap::new();

    let mut current: i64 = 0;
    while (current as usize) < program.len() {
        let mut word_iter = program[current as usize].split_whitespace();
        let cmd = word_iter.next().unwrap();
        // println!(
        //     "Current state: {:?} - executing {} at line {}",
        //     registers, cmd, current
        // );
        let mut move_step = 1;
        match cmd {
            "mov" => {
                let reg_key = String::from(word_iter.next().unwrap());
                let check = word_iter.next().unwrap();
                let val = if check.chars().next().unwrap().is_ascii_alphabetic() {
                    registers[check]
                } else {
                    i64::from_str(check).unwrap()
                };

                registers.insert(reg_key, val);
            }
            "inc" => {
                registers
                    .entry(String::from(word_iter.next().unwrap()))
                    .and_modify(|x| *x += 1);
            }
            "dec" => {
                registers
                    .entry(String::from(word_iter.next().unwrap()))
                    .and_modify(|x| *x -= 1);
            }
            "jnz" => {
                let check = word_iter.next().unwrap();
                let jump_size = i64::from_str(word_iter.next().unwrap()).unwrap();
                let val = if check.chars().next().unwrap().is_ascii_alphabetic() {
                    registers[check]
                } else {
                    i64::from_str(check).unwrap()
                };
                if val != 0 {
                    move_step = jump_size;
                }
            }
            _ => panic!("unknown command"),
        }
        current += move_step;
    }
    registers
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}
