extern crate rand;

fn main() {
    let x: u8 = rand::random();
    println!("Random u8: {}", x);
    println!("As roman numerals: {}", roman_numerals(x as u32));
}

fn roman_numerals(integer: u32) -> String {
    let numerals = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I")
    ];

    let mut remaining_numerals = integer;
    let mut roman_numerals = String::from("");
    for (numeral_value, numeral) in &numerals {
        while remaining_numerals >= *numeral_value {
            roman_numerals.push_str(*numeral);
            remaining_numerals -= *numeral_value;
        }
    }
    return roman_numerals;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_romal_numerals() {
        assert_eq!("",    roman_numerals(0));
        assert_eq!("I",   roman_numerals(1));
        assert_eq!("II",  roman_numerals(2));
        assert_eq!("III", roman_numerals(3));
        assert_eq!("IV",  roman_numerals(4));
        assert_eq!("V",   roman_numerals(5));
        assert_eq!("VI",  roman_numerals(6));
        assert_eq!("IX",  roman_numerals(9));
        assert_eq!("X",   roman_numerals(10));
        assert_eq!("XI",  roman_numerals(11));
        assert_eq!("XX",  roman_numerals(20));
        assert_eq!("XX",  roman_numerals(20));
        assert_eq!("XL",  roman_numerals(40));
        assert_eq!("L",   roman_numerals(50));
        assert_eq!("LX",  roman_numerals(60));
        assert_eq!("XC",  roman_numerals(90));
        assert_eq!("C",   roman_numerals(100));
        assert_eq!("CD",  roman_numerals(400));
        assert_eq!("D",   roman_numerals(500));
        assert_eq!("CM",  roman_numerals(900));
        assert_eq!("M",   roman_numerals(1000));
    }

    #[test]
    fn complex_roman_numeral() {
        assert_eq!("MMMCMXCIX", roman_numerals(3999));
    }
}
