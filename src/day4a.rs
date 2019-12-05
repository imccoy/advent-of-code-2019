use std::io;

const MIN : [i32;6] = [0;6];
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

fn count_combinations(number : i32, min: &[i32], max: &[i32], previous_digit : i32, place : i32, have_repeated : bool) -> i32 {
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
        println!("{} <= n <= {}", min[0], max[0]);
        if min[0] == max[0] {
            let current_digit = min[0];
            return count_combinations(number * 10 + current_digit, &min[1..], &max[1..], current_digit, place + 1, have_repeated || (current_digit == previous_digit));
        } else {
            if (min[0] >= previous_digit) {
                let mut count = 0;
                count += count_combinations(number * 10 + min[0], &min[1..], &MAX, min[0], place + 1, (place != 0 && min[0] == previous_digit) || have_repeated);
                if max[0] - min[0] > 1 {
                    for current_digit in (min[0]+1)..max[0] {
                        count += count_combinations(number * 10 + current_digit, &MIN, &MAX, current_digit, place + 1, (place != 0 && current_digit == previous_digit) ||have_repeated);
                    }
                }
                count += count_combinations(number * 10 + max[0], &MIN, &max[1..], max[0], place + 1, (place != 0 && max[0] == previous_digit) || have_repeated);
                return count;
            } else if (previous_digit < max[0]) {
                let mut count = 0;
                count += count_combinations(number * 10 + previous_digit, &MIN, &MAX, previous_digit, place + 1, true);
                if max[0] - previous_digit > 1 {
                    for current_digit in (previous_digit+1)..max[0] {
                        count += count_combinations(number * 10 + current_digit, &MIN, &MAX, current_digit, place + 1, (place != 0 && current_digit == previous_digit) ||have_repeated);
                    }
                }
                count += count_combinations(number * 10 + max[0], &MIN, &max[1..], max[0], place + 1, (place != 0 && max[0] == previous_digit) || have_repeated);
                return count;
            } else if (previous_digit == max[0]) {
                let current_digit = max[0];
                return count_combinations(number * 10 + current_digit, &MIN, &max[1..], current_digit, place + 1, have_repeated || (current_digit == previous_digit));
            } else {
                return 0;
            }
        }
    }
}

fn main() -> io::Result<()> {
    println!("{}", count_combinations(0, &[1,3,7,6,8,3], &[5,9,6,2,5,3], 0, 0, false));
    Ok(())
}
