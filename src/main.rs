use std::thread;

const NUM_THREADS: u32 = 4;
const MAX_VALUE: usize = 20;

#[inline]
fn multiply_digits(number: u32) -> u32 {
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
fn multiplicative_persistence(number: u32) -> u8 {
    let mut persistence = 0;
    let mut current = number;
    while current > 9 {
        persistence += 1;
        current = multiply_digits(current);
    }
    persistence
}

fn persistence_range(start: u32, end: u32) -> [u32; MAX_VALUE] {
    let mut current = start;
    let mut top_array = [0u32; MAX_VALUE];

    while current < end {
        let persistence = multiplicative_persistence(current) as usize;
        if top_array[persistence] == 0 {
            top_array[persistence] = current;
        }
        current += 1;
    }
    top_array
}

fn main() {
    let step = !0 / NUM_THREADS;
    let mut threads = vec![];

    let mut current = 0u32;
    for i in 0..NUM_THREADS {
        threads.push(thread::spawn(move || -> [u32; 20] {
            persistence_range(current, current + step)
        }));
        if i != NUM_THREADS - 1 {
            current += step;
        }
    }

    let mut intermediate_arrays = vec![];
    for thread in threads {
        intermediate_arrays.push(thread.join().unwrap());
    }

    let mut final_array = [0u32; MAX_VALUE];
    for intermediate_array in intermediate_arrays {
        for i in 0..MAX_VALUE {
            if final_array[i] == 0 {
                final_array[i] = intermediate_array[i];
            }
        }
    }
    println!("{:?}", final_array);
}

#[cfg(test)]
mod tests {
    use crate::{multiplicative_persistence, multiply_digits, persistence_range};
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
    fn when_multiply_digits_called_then_correct_value_returned(number: u32, expected: u32) {
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
        case(3778888999, 10)
    )]
    fn when_multiplicative_persistence_called_then_correct_value_returned(
        number: u32,
        expected: u8,
    ) {
        assert_eq!(multiplicative_persistence(number), expected);
    }

    #[test]
    fn when_persistence_range_then_correct_value_returned() {
        let expected = [
            0, 65540, 65536, 66111, 66118, 66177, 66999, 68889, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(persistence_range(1 << 16, 1 << 17), expected);
    }
}
