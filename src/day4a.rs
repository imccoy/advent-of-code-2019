use std::io;

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

fn count_combinations_0() -> i32 {
    let mut count = 0;
    for digit in 0..10 {
        count += count_combinations(digit, digit, 1, false);
    }
    return count;
}

fn count_combinations(number : i32, current_digit : i32, place : i32, have_repeated : bool) -> i32 {
    if place == 5 {
        // if we're in the last position, and we haven't repeated a digit yet, we'll have to repeat the previous digit.
        // but if we've already covered the repeat-a-digit requirement, we'll have a bunch of options here
        if have_repeated {
            println!("{}x", number);
            return 10 - current_digit;
        } else {
            println!("{}{}", number, current_digit);
            return 1;
        }
    } else {
        let mut count = 0;
        count += count_combinations(number * 10 + current_digit, current_digit, place + 1, true);
        for next_digit in (current_digit+1)..10 {
            count += count_combinations(number * 10 + next_digit, next_digit, place + 1, have_repeated);
        }
        return count;
    }
}

fn main() -> io::Result<()> {
    println!("{}", count_combinations_0());
    Ok(())
}
