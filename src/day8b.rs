
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let width = 25;
    let height = 6;
    let image_size = width * height;
    let digits : Vec<char> = io::stdin().lock().lines().next().unwrap()?.chars().collect();
    let layers : Vec<&[char]> = digits.chunks(image_size).collect();
    let result : Vec<char> = (0..image_size).map(|index| {
        for layer in &layers {
            let colour = layer[index];
            if colour == '1' {
                return 'X';
            } else if colour == '0' {
                return ' ';
            }
        }
        return '2';
    }).collect();

    for line in result.chunks(width) {
        let line_string : String = line.iter().collect();
        println!("{}", line_string);
    }
    Ok(())
}
