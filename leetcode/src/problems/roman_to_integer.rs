pub struct Roman {
    pub value: Vec<RomanChar>,
}

impl Roman {
    pub fn new(roman_str: String) -> Result<Self, &'static str> {
        let mut roman_char_vec = Vec::<RomanChar>::new();
        for c in roman_str.chars().into_iter() {
            match RomanChar::new(c) {
                Ok(val) => {
                    roman_char_vec.push(val);
                }
                Err(_) => {
                    return Err("argument contains invalid charactor.");
                }
            }
        }

        Ok(Self {
            value: roman_char_vec,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RomanChar {
    pub value: char,
}

impl RomanChar {
    pub fn new(value: char) -> Result<Self, &'static str> {
        match value {
            'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' => Ok(Self { value }),
            _ => Err("Invalid Char."),
        }
    }
}

pub struct RomanParser {}

impl RomanParser {
    pub fn parse(&self, roman: Roman) -> usize {
        let mut result: usize = 0;
        // 一文字ずつ進めてresultに足していく
        // 一文字前の文字を覚えておいて、その文字によって足す値が変わる
        let mut before: Option<RomanChar> = None;
        for current in roman.value.into_iter() {
            result += Self::parse_one_char(current.clone(), before);
            before = Some(current.clone());
        }
        result
    }

    fn parse_one_char(current: RomanChar, before: Option<RomanChar>) -> usize {
        match current.value {
            'I' => 1,
            'V' => match before {
                Some(val) => {
                    if val.value == 'I' {
                        3
                    } else {
                        5
                    }
                }
                None => 5,
            },

            'X' => match before {
                Some(val) => {
                    if val.value == 'I' {
                        8
                    } else {
                        10
                    }
                }
                None => 10,
            },

            'L' => match before {
                Some(val) => {
                    if val.value == 'X' {
                        30
                    } else {
                        50
                    }
                }
                None => 50,
            },

            'C' => match before {
                Some(val) => {
                    if val.value == 'X' {
                        80
                    } else {
                        100
                    }
                }
                None => 100,
            },

            'D' => match before {
                Some(val) => {
                    if val.value == 'C' {
                        300
                    } else {
                        500
                    }
                }
                None => 500,
            },

            'M' => match before {
                Some(val) => {
                    if val.value == 'C' {
                        800
                    } else {
                        1000
                    }
                }
                None => 1000,
            },

            _ => 0,
        }
    }
}
pub struct Solution {}

impl Solution {
    pub fn roman_to_int(&self, roman_str: String) -> usize {
        let roman_parser = RomanParser {};
        let roman = Roman::new(roman_str).unwrap();
        roman_parser.parse(roman)
    }
}

fn main() {
    let roman_str = String::from("III");
    let solution = Solution {};
    let result = solution.roman_to_int(roman_str);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let solution = Solution {};
        let test_case_1 = "III".to_string();
        assert_eq!(solution.roman_to_int(test_case_1), 3);

        let test_case_2 = "LVIII".to_string();
        assert_eq!(solution.roman_to_int(test_case_2), 58);

        let test_case_3 = "MCMXCIV".to_string();
        assert_eq!(solution.roman_to_int(test_case_3), 1994);
    }
}
