use std::io;

const MIN : [i32;6] = [0;6];
const MAX : [i32;6] = [9;6];

#[derive(PartialEq, Copy, Clone, Debug)]
enum Repeated { NotStarted, One, Two, Golden, Gone }

fn repeating_digit(repeated : Repeated, previous_digit : i32, new_digit : i32) -> Repeated {
    match repeated {
        Repeated::Golden => Repeated::Golden,
        Repeated::NotStarted => Repeated::One,
        Repeated::One => if previous_digit == new_digit {
                   Repeated::Two
               } else {
                   Repeated::One
               }
        Repeated::Two => if previous_digit == new_digit {
                   Repeated::Gone
               } else {
                   Repeated::Golden
               }
        Repeated::Gone => if previous_digit == new_digit {
                   Repeated::Gone
               } else {
                   Repeated::One
               }
    }
}

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

fn count_combinations(number : i32, min: &[i32], max: &[i32], previous_digit : i32, place : i32, repeated : Repeated) -> i32 {
    if place == 5 {
        // if we're in the last position, and we haven't repeated a digit yet, we'll have to repeat the previous digit.
        // but if we've already covered the repeat-a-digit requirement, we'll have a bunch of options here
        if repeated == Repeated::Golden {
            println!("{}x ({}) Golden", number, max[0] - previous_digit + 1);
            return max[0] - previous_digit + 1;
        } else if repeated == Repeated::Two {
            println!("{}x ({}) Two", number, max[0] - previous_digit);
            return max[0] - previous_digit;
        } else if repeated == Repeated::One {
            println!("{}{}", number, previous_digit);
            return 1;
        } else if repeated == Repeated::Gone {
            return 0;
        } else {
            return 0;
        }
    } else {
        println!("{} <= n <= {}", min[0], max[0]);
        if min[0] == max[0] {
            let current_digit = min[0];
            return count_combinations(number * 10 + current_digit, &min[1..], &max[1..], current_digit, place + 1, repeating_digit(repeated, previous_digit, current_digit));
        } else {
            if (previous_digit == max[0]) {
                let current_digit = max[0];
                return count_combinations(number * 10 + current_digit, &MIN, &max[1..], current_digit, place + 1, repeating_digit(repeated, previous_digit, current_digit));
            } else if (min[0] >= previous_digit) {
                let mut count = 0;
                count += count_combinations(number * 10 + min[0], &min[1..], &MAX, min[0], place + 1, repeating_digit(repeated, previous_digit, min[0]));
                if max[0] - min[0] > 1 {
                    for current_digit in (min[0]+1)..max[0] {
                        count += count_combinations(number * 10 + current_digit, &MIN, &MAX, current_digit, place + 1, repeating_digit(repeated, previous_digit, current_digit));
                    }
                }
                count += count_combinations(number * 10 + max[0], &MIN, &max[1..], max[0], place + 1, repeating_digit(repeated, previous_digit, max[0]));
                return count;
            } else if (previous_digit < max[0]) {
                let mut count = 0;
                count += count_combinations(number * 10 + previous_digit, &MIN, &MAX, previous_digit, place + 1, repeating_digit(repeated, previous_digit, previous_digit));
                if max[0] - previous_digit > 1 {
                    for current_digit in (previous_digit+1)..max[0] {
                        count += count_combinations(number * 10 + current_digit, &MIN, &MAX, current_digit, place + 1, repeating_digit(repeated, previous_digit ,current_digit));
                    }
                }
                count += count_combinations(number * 10 + max[0], &MIN, &max[1..], max[0], place + 1, repeating_digit(repeated, previous_digit, max[0]));
                return count;
            } else {
                return 0;
            }
        }
    }
}

fn main() -> io::Result<()> {
    println!("{}", count_combinations(0, &[1,3,7,6,8,3], &[5,9,6,2,5,3], 0, 0, Repeated::NotStarted));
    Ok(())
}