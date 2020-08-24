lazy_static! {

    static ref SPIRAL_TWO:  Vec<Vec<i8>> = //like this
    vec![
        vec![1,1],
        vec![0,1]
    ];
static ref SPIRAL_THREE:  Vec<Vec<i8>> = //like this
    vec![
        vec![1,1,1],
        vec![0,0,1],
        vec![1,1,1],
    ];
static ref SPIRAL_FOUR: Vec<Vec<i8>> = //like this
    vec![
        vec![1,1,1,1],
        vec![0,0,0,1],
        vec![1,0,0,1],
        vec![1,1,1,1],
    ];
static ref SPIRAL_FIVE:  Vec<Vec<i8>> = //like this
    vec![
        vec![1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 1],
        vec![1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ];

}

fn easy_print(v: &[Vec<i8>]) {
    for row in v {
        println!("{:?}", row);
    }
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    //recursion!
    match size {
        0 | 1 => unreachable!(),
        2 => SPIRAL_TWO.clone(),
        3 => SPIRAL_THREE.clone(),
        4 => SPIRAL_FOUR.clone(),
        5 => SPIRAL_FIVE.clone(),
        n => {
            let mut res = vec![vec![1i8; n]; n];
            res[1][0] = 0;
            let middle = vec![0i8; n - 2];

            res[1][1..n - 1].copy_from_slice(&middle);
            res[n - 2][1..n - 1].copy_from_slice(&middle);
            for (i, row) in spiralize(n - 4).iter().enumerate() {
                res[i + 2][1] = 0;
                res[i + 2][2..n - 2].copy_from_slice(row);
                res[i + 2][n - 2] = 0;
            }
            res[2][1] = 1;
            res
        }
    }
}

fn spiralize_smaller(size: usize) -> Vec<Vec<i8>> {
    let mut spiral = vec![vec![0; size]; size];
    let mut value = 1;

    for j in 0..(size + 1) / 2 {
        for i in j..(size - j) {
            spiral[i][j] = value;
            spiral[j][i] = value;

            spiral[i][size - 1 - j] = value;
            spiral[size - 1 - j][i] = value;
        }

        value = (value + 1) % 2;

        if j < (size - 1) / 2 || spiral[j][j - 1] == 1 {
            spiral[j + 1][j] = value;
        }
    }

    spiral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }
    #[test]
    fn test20() {
        let expected = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(spiralize(20), expected);
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
