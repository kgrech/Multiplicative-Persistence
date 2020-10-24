const MAX_VALUE: usize = 20;
const NUM_DIGITS: usize = 38;

#[inline]
fn multiply_digits_array(digit: &[u8; NUM_DIGITS]) -> u128 {
    let mut product = 1u128;
    let mut i = NUM_DIGITS - 1;
    while i > 0 && digit[i] != 1 {
        product *= digit[i] as u128;
        i -= 1;
    }
    product * digit[i] as u128
}

#[inline]
fn multiply_digits(number: u128) -> u128 {
    if number == 0 {
        return 0;
    }
    let mut result = 1;
    let mut rest = number;
    while rest != 0 {
        result *= rest % 10;
        rest /= 10;
    }
    result
}

#[inline]
fn multiplicative_persistence(number: u128) -> u8 {
    let mut persistence = 0;
    let mut current = number;
    while current > 9 {
        persistence += 1;
        current = multiply_digits(current);
    }
    persistence
}

#[inline]
fn multiplicative_persistence_array(digit: &[u8; NUM_DIGITS]) -> u8 {
    let next_number = multiply_digits_array(digit);
    multiplicative_persistence(next_number) + 1
}

fn inc(digit: &mut [u8; NUM_DIGITS]) -> bool {
    let mut i = NUM_DIGITS - 1;
    while i > 0 && digit[i] == 9 {
        i -= 1;
    }
    if digit[i] != 9 {
        digit[i] += 1;
        for j in i + 1..NUM_DIGITS {
            digit[j] = digit[i];
        }
        return true;
    }
    false
}

fn as_number(array: &[u8; NUM_DIGITS]) -> u128 {
    let mut digit = 0u128;
    for d in array.iter() {
        if *d != 1 {
            digit = 10 * digit + *d as u128;
        }
    }
    digit
}

fn main() {
    let mut digit = [1u8; NUM_DIGITS];
    let mut top_array = [0u128; MAX_VALUE];

    while inc(&mut digit) {
        let persistence = multiplicative_persistence_array(&digit) as usize - 1;
        if top_array[persistence] == 0 {
            top_array[persistence] = as_number(&digit);
        }
    }
    println!("{:?}", top_array)
}

#[cfg(test)]
mod tests {
    use crate::{
        multiplicative_persistence, multiplicative_persistence_array, multiply_digits, NUM_DIGITS,
    };
    use rstest::rstest;

    #[rstest(
        number,
        expected,
        case(0, 0),
        case(128, 16),
        case(999, 729),
        case(1081, 0),
        case(4294967295, 9797760)
    )]
    fn when_multiply_digits_called_then_correct_value_returned(number: u128, expected: u128) {
        assert_eq!(multiply_digits(number), expected);
    }

    #[rstest(
        number,
        expected,
        case(2, 0),
        case(11, 1),
        case(29, 2),
        case(47, 3),
        case(277, 4),
        case(769, 5),
        case(8867, 6),
        case(186889, 7),
        case(2678789, 8),
        case(26899889, 9),
        case(3778888999, 10),
        case(277777788888899, 11)
    )]
    fn when_multiplicative_persistence_called_then_correct_value_returned(
        number: u128,
        expected: u8,
    ) {
        assert_eq!(multiplicative_persistence(number), expected);
    }

    #[rstest(
        number,
        expected,
        case(29, 2),
        case(47, 3),
        case(277, 4),
        case(769, 5),
        case(8867, 6),
        case(186889, 7),
        case(2678789, 8),
        case(26899889, 9),
        case(3778888999, 10),
        case(277777788888899, 11)
    )]
    fn when_multiplicative_persistence_array_called_then_correct_value_returned(
        number: u128,
        expected: u8,
    ) {
        let mut digit = [1u8; NUM_DIGITS];
        let mut rest = number;
        let mut i = NUM_DIGITS - 1;
        while rest != 0 {
            digit[i] = (rest % 10) as u8;
            rest /= 10;
            i -= 1;
        }
        assert_eq!(multiplicative_persistence_array(&digit), expected);
    }
}
