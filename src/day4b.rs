use std::io;

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

fn count_combinations(number : i32, min: Option<&[i32]>, max: Option<&[i32]>, previous_digit : i32, place : i32, repeated : Repeated) -> i32 {
    let (max_here, max_rest) = max.map(|max_digits| (max_digits[0], Some(&max_digits[1..]))).unwrap_or((9, None));
    if place == 5 {
        // if we're in the last position, and we haven't repeated a digit yet, we'll have to repeat the previous digit.
        // but if we've already covered the repeat-a-digit requirement, we'll have a bunch of options here
        if repeated == Repeated::Golden {
            println!("{}x ({}) Golden", number, max_here - previous_digit + 1);
            return max_here - previous_digit + 1;
        } else if repeated == Repeated::Two {
            println!("{}x ({}) Two", number, max_here - previous_digit);
            return max_here - previous_digit;
        } else if repeated == Repeated::One {
            println!("{}{}", number, previous_digit);
            return 1;
        } else if repeated == Repeated::Gone {
            return 0;
        } else {
            return 0;
        }
    } else {
        let (min_here, min_rest) = min
            .and_then(|min_digits| {
                if min_digits[0] >= previous_digit {
                    Some ((min_digits[0], Some(&min_digits[1..])))
                } else {
                    None
                }
            }).unwrap_or((previous_digit, None));


          
        if min_here == max_here {
           let current_digit = min_here;
           return count_combinations(number * 10 + current_digit, min_rest, max_rest, current_digit, place + 1, repeating_digit(repeated, previous_digit, current_digit));
        } else if min_here < max_here {
            let mut count = 0;
            count += count_combinations(number * 10 + min_here, min_rest, None, min_here, place + 1, repeating_digit(repeated, previous_digit, min_here));
            if max_here - min_here > 1 {
                for current_digit in (min_here+1)..max_here {
                    count += count_combinations(number * 10 + current_digit, None, None, current_digit, place + 1, repeating_digit(repeated, previous_digit, current_digit));
                }
            }
            count += count_combinations(number * 10 + max_here, None, max_rest, max_here, place + 1, repeating_digit(repeated, previous_digit, max_here));
            return count;
        } else {
            return 0;
        }
    }
}

fn main() -> io::Result<()> {
    println!("{}", count_combinations(0, Some(&[1,3,7,6,8,3]), Some(&[5,9,6,2,5,3]), 0, 0, Repeated::NotStarted));
    Ok(())
}
