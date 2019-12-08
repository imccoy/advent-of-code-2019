
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


    // just for giggles, do it again but without explicitly collecting the layers into a vec
    let mut result : [char;150] = ['2';150];
    for layer in digits.chunks(image_size) {
        for (index, value) in layer.iter().enumerate() {
            if result[index] == '2' {
                result[index] = match *value {
                                    '0' => ' ',
                                    '1' => 'X',
                                    _   => '2'
                                }
            }
        }
    }

    for line in result.chunks(width) {
        let line_string : String = line.iter().collect();
        println!("{}", line_string);
    }

    Ok(())
}
