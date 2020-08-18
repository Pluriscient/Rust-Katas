mod solution {

    pub fn range_extraction(a: &[i32]) -> String {
        // Your solution here
        let mut res: Vec<String> = vec![];
        let mut prevs = vec![];
        for x in a.iter() {
            if prevs.len() == 0 {
                prevs.push(x);
                continue;
            }
            let &prev = prevs.last().unwrap();
            let gap = x - prev;
            if gap == 1 {
                prevs.push(x);
            } else if gap == 0 {
                unreachable!();
            } else {
                res.push(match prevs.len() {
                    1 => prev.to_string(),
                    2 => format!("{},{}", prevs.first().unwrap(), prev),
                    0 => unreachable!(),
                    _ => format!("{}-{}", prevs.first().unwrap(), prev),
                });

                prevs.clear();
                prevs.push(x);
            }
        }
        match prevs.len() {
            0 => (),
            1 => res.push(prevs.first().unwrap().to_string()),
            2 => prevs.iter().for_each(|x| res.push(x.to_string())),
            _ => res.push(format!(
                "{}-{}",
                prevs.first().unwrap(),
                prevs.last().unwrap()
            )),
        }
        res.join(",")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        // assert_eq!(
        //     "-3--1,2,10,15,16,18-20",
        //     solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        // );
    }
}
