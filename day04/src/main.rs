use ToString;

type Password = u32;

fn get_digits(password: Password) -> Vec<u32> {
    password
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

fn is_valid(password: Password) -> bool {
    let digits = get_digits(password);
    is_monotone_increasing(&digits) && has_six_digits(&digits) && has_two_adjacent_digits(&digits)
}

fn is_strictly_valid(password: Password) -> bool {
    let digits = get_digits(password);
    is_valid(password) && has_two_isolated_adjacent_digits(&digits)
}

fn is_monotone_increasing(digits: &[u32]) -> bool {
    let mut digits = digits.into_iter();

    let mut previous_digit = digits.next().unwrap();
    for digit in digits {
        if previous_digit > digit {
            return false;
        }
        previous_digit = digit;
    }
    true
}

fn has_six_digits(digits: &[u32]) -> bool {
    digits.len() == 6
}

fn has_two_adjacent_digits(digits: &[u32]) -> bool {
    let mut digits = digits.into_iter();

    let mut previous_digit = digits.next().unwrap();
    for digit in digits {
        if previous_digit == digit {
            return true;
        }
        previous_digit = digit;
    }
    false
}

fn has_two_isolated_adjacent_digits(digits: &[u32]) -> bool {
    let mut digits = digits.into_iter();

    let mut current_digit = digits.next().unwrap();
    let mut group_size = 1;
    for next_digit in digits {
        if current_digit == next_digit {
            group_size += 1;
        } else if group_size == 2 {
            return true;
        } else {
            current_digit = next_digit;
            group_size = 1;
        }
    }
    group_size == 2
}

fn main() {
    let password_range = 272091..=815432;
    // Brute-force solution - part 1
    let mut n_valid_password = 0;
    for password in password_range.clone() {
        if is_valid(password) {
            n_valid_password += 1;
        }
    }
    println!("There are {} valid passwords!", n_valid_password);

    // Brute-force solution - part 2
    let mut n_strictly_valid_password = 0;
    for password in password_range {
        if is_strictly_valid(password) {
            n_strictly_valid_password += 1;
        }
    }
    println!("There are {} strictly valid passwords!", n_strictly_valid_password);
}

#[cfg(test)]
mod tests {
    use crate::is_strictly_valid;

    #[test]
    fn strictly_valid_passwords() {
        assert!(is_strictly_valid(112233));
        assert!(is_strictly_valid(111122));
        assert!(!is_strictly_valid(123444));
    }
}
