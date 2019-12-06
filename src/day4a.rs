use std::io;

const MAX : [i32;6] = [9;6];

fn digits_from_num(num : i32) -> [i32;6] {
    let mut num = num;
    let mut digits : [i32;6] = [0;6];
    let mut place = 5;
    while num > 0 {
        digits[place] = num % 10;
        place -= 1;
        num /= 10;
    }
    return digits;
}

fn count_combinations(number : i32, min: Option<&[i32]>, max: &[i32], previous_digit : i32, place : i32, have_repeated : bool) -> i32 {
    if place == 5 {
        // if we're in the last position, and we haven't repeated a digit yet, we'll have to repeat the previous digit.
        // but if we've already covered the repeat-a-digit requirement, we'll have a bunch of options here
        if have_repeated {
            println!("{}x ({})", number, max[0] - previous_digit + 1);
            return max[0] - previous_digit + 1;
        } else {
            println!("{}{}", number, previous_digit);
            return 1;
        }
    } else {
        let (first_possible_digit, min_rest) = min
            .and_then(|min_digits| {
                if min_digits[0] >= previous_digit {
                    Some ((min_digits[0], Some(&min_digits[1..])))
                } else {
                    None
                }
            }).unwrap_or((previous_digit, None));

        if first_possible_digit == max[0] {
            let current_digit = first_possible_digit;
            return count_combinations(number * 10 + current_digit, min_rest, &max[1..], current_digit, place + 1, have_repeated || (current_digit == previous_digit));
        } else if (first_possible_digit < max[0]) {
            let mut count = 0;
            count += count_combinations(number * 10 + first_possible_digit, min_rest, &MAX, first_possible_digit, place + 1, (place != 0 && first_possible_digit == previous_digit) ||have_repeated);
            if max[0] - first_possible_digit > 1 {
                for current_digit in (first_possible_digit+1)..max[0] {
                    count += count_combinations(number * 10 + current_digit, None, &MAX, current_digit, place + 1, (place != 0 && current_digit == previous_digit) ||have_repeated);
                }
            }
            count += count_combinations(number * 10 + max[0], None, &max[1..], max[0], place + 1, (place != 0 && max[0] == previous_digit) || have_repeated);
            return count;
        } else {
            return 0;
        }
    }
}

fn main() -> io::Result<()> {
    println!("{}", count_combinations(0, Some(&[1,3,7,6,8,3]), &[5,9,6,2,5,3], 0, 0, false));
    Ok(())
}
