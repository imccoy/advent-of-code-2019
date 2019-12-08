
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let width = 25;
    let height = 6;
    let image_size = width * height;
    let digits : Vec<char> = io::stdin().lock().lines().next().unwrap()?.chars().collect();
    let layers = digits.chunks(image_size);
    let annotated_layers = layers.map(|layer| (layer.iter().filter(|&&layer_digit| layer_digit == '0').count(), layer));

    let best_layer = annotated_layers.min_by(|(zeros_count_a, _), (zeros_count_b, _)| zeros_count_a.cmp(zeros_count_b));

    println!("{:?}", best_layer.map(|(_, layer)| {
        let ones_count = layer.iter().filter(|&&layer_digit| layer_digit == '1').count();
        let twos_count = layer.iter().filter(|&&layer_digit| layer_digit == '2').count();
        ones_count * twos_count
    }));
    Ok(())
}
