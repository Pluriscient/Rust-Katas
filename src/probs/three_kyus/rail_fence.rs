fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut res: Vec<char> = vec![];
    let chars = text.as_bytes();
    let mut jump = 2 * (num_rails - 1);

    for rail in 0..(num_rails) {
        res.extend_from_slice(
            &(rail..chars.len())
                .step_by(jump)
                .map(|i| chars[i] as char)
                .collect::<Vec<char>>(),
        );
        jump -= 2;
        if jump == 0 {
            // reset jump for last iteration
            jump = 2 * (num_rails - 1);
        }
    }
    res.into_iter().collect()
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut res: Vec<char> = vec!['-'; text.len()];
    let chars: Vec<char> = text.chars().collect();
    // let mut jump
    let mut index = 0;
    let mut jump = 2 * (num_rails - 1);

    for rail in 0..num_rails {
        for i in (rail..chars.len()).step_by(jump) {
            res[index] = chars[i];
            index += 1;
        }
        jump -= 2;
        if jump == 0 {
            // reset jump for last iteration
            jump = 2 * (num_rails - 1);
        }
    }

    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_encode() {
        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
    }
    #[test]
    fn basic_decode() {
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }

    #[test]
    fn basic_tests() {
        assert_eq!(
            decode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}
