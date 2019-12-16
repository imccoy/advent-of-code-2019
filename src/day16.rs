use std::convert::TryFrom;
use std::cmp::min;
use std::io::{self,BufRead};


#[derive(Clone)]
struct RepeatEach<T, I>
    where 
        I: Iterator<Item = T>,
        T: Copy
{
    times : usize,
    iter: I,
    current_element: Option<T>,
    current_n: usize
}

impl<T, I> Iterator for RepeatEach<T, I> 
    where
        I: Iterator<Item = T>,
        T: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.current_element {
            None => {
                self.current_element = self.iter.next();
                self.current_n = 1;
                return self.current_element;
            },
            Some(e) => {
                self.current_n += 1;
                if self.current_n > self.times {
                    self.current_element = None;
                    return self.next();
                }
                return Some(e);
            }
        }
    }
}

fn repeat_each<T : Copy, I : Iterator<Item = T>>(iter: I, num: usize) -> RepeatEach<T, I> {
    return RepeatEach { iter: iter, times: num, current_element: None, current_n: 0};
}

fn main() -> io::Result<()> {
    let digit_string = io::stdin().lock().lines().next().unwrap()?;
    let input_digits : Vec<i32> = digit_string
      .chars()
      .map(|char| char.to_digit(10))
      .filter_map(|maybe_digit| maybe_digit.and_then(|digit| i32::try_from(digit).ok()))
      .collect();
    let mut digits : Vec<i32> = input_digits.clone();
    println!("{:?}", digits.len());
    let base_pattern : [i32;4] = [0, 1, 0, -1];
    for iter in 0..100 {
        println!("{:?}", iter);
        digits = (1..(digits.len()+1)).map(|pattern_reps| {
            let positive_start : i32 = pattern_reps as i32;
            let positive_end : i32 = (pattern_reps * 2) as i32;
            let negative_start : i32 = (pattern_reps * 3) as i32;
            let negative_end : i32 = (pattern_reps * 4) as i32;
            let mut offset : i32 = -1;
            let mut n : i32 = 0;
            loop {
                if offset + positive_start >= i32::try_from(digits.len()).unwrap() {
                    break;
                }
                n += digits[usize::try_from(offset + positive_start).unwrap()..min(usize::try_from(offset + positive_end).unwrap(), digits.len())].iter().sum::<i32>();
                if offset + negative_start >= i32::try_from(digits.len()).unwrap() {
                    break;
                }
                n -= digits[usize::try_from(offset + negative_start).unwrap()..min(usize::try_from(offset + negative_end).unwrap(), digits.len())].iter().sum::<i32>();
                offset += negative_end;
            }
            (n % 10).abs()
        }).collect();
    }
    println!("{:?}", digits);
    Ok(())
}

