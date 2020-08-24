use std::collections::HashMap;

struct MorseDecoder<'r> {
    morse_code: HashMap<&'r str, &'r str>,
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
impl MorseDecoder<'_> {
    pub fn new() -> Self {
        MorseDecoder {
            morse_code: hashmap![".-.."=> "L", "--"=> "M", "-"=> "T", "..-"=> "U", ".--"=> "W", "--..."=> "7", "-.-.--"=> "!", "..--.-"=> "_", ".-..-."=> "\"", "---..."=> ",", "-.-"=> "K", "---"=> "O", "...-"=> "V", "-..."=> "B", "-----"=> "0", "..-."=> "F", ".-"=> "A", "...."=> "H", "-."=> "N", ".-."=> "R", "---.."=> "8", "----."=> "9", ".----."=> "\'", "..---"=> "2", "..--.."=> "?", "-.."=> "D", "-.-."=> "C", "..."=> "S", "....-"=> "4", "--..--"=> ",", ".----"=> "1", "-..-."=> "/", "-.--"=> "Y", "-.--."=> "(", "."=> "E", ".--."=> "P", ".-.-.-"=> ".", "--.-"=> "Q", "-.-.-."=> ";", "-....-"=> "-", "...-..-"=> "$", "....."=> "5", ".."=> "I", ".-..."=> "&", "-..-"=> "X", "--.."=> "Z", "...--"=> "3", ".-.-."=> "+", ".--.-."=> "@", "...---..."=> "SOS", "--."=> "G", ".---"=> "J", "-...."=> "6", "-...-"=> "=", "-.--.-"=> ")"],
        }
    }

    const DOT: i64 = 1;
    const DASH: i64 = 3;
    const INNER_CHARACTER_PAUSE: i64 = -1;
    const OUTER_CHARACTER_PAUSE: i64 = -3;
    const WORD_PAUSE: i64 = -7;
    pub fn decode_bits(&self, encoded: &str) -> String {
        let sequences = {
            let mut res: Vec<i64> = vec![];
            let mut cur: i64 = 0;
            for c in encoded.chars() {
                match (c, cur) {
                    ('0', x) if x <= 0 => cur -= 1,
                    ('1', x) if x >= 0 => cur += 1,
                    ('0', x) => {
                        res.push(x);
                        cur = -1;
                    }
                    ('1', x) => {
                        res.push(x);
                        cur = 1;
                    }
                    (_, _) => unreachable!(),
                }
            }
            if cur > 0 {
                res.push(cur)
            }
            if !res.is_empty() {
                if res[0] < 0 {
                    res.remove(0);
                }
            }
            res
        };
        let unit = sequences.iter().map(|x| x.abs()).min().unwrap();

        let mappings: HashMap<i64, &str> = [
            (MorseDecoder::DOT * unit, "."),
            (MorseDecoder::DASH * unit, "-"),
            (MorseDecoder::INNER_CHARACTER_PAUSE * unit, ""),
            (MorseDecoder::OUTER_CHARACTER_PAUSE * unit, " "),
            (MorseDecoder::WORD_PAUSE * unit, "   "),
        ]
        .iter()
        .cloned()
        .collect();

        println!("mapping {:?} using {:?}", sequences, mappings);
        sequences
            .iter()
            .map(|x| {
                let entry = mappings.get(&x);
                match entry {
                    None => panic!("Could not find entry for {}", x),
                    Some(found) => found.to_string(),
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        println!("decoding {}", encoded);
        let words = encoded.split("   ");
        words
            .map(|word| {
                let characters = word.split(" ");

                characters
                    .filter(|x| !x.is_empty())
                    .map(|mcode| match self.morse_code.get(mcode) {
                        Some(found) => found.to_string(),
                        None => panic!("Could not find entry for '{}'", mcode),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

mod morse_testing {
    use super::*;
    #[test]
    fn short_basic() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits("101")), "a")
    }
    #[test]
    fn short_basic_2() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&" . "), "a")
    }

    #[test]
    fn examples() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
    }
}
