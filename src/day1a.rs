use std::io::{self, BufRead};

fn fuel_for_mass(mass: i32) -> i32 {
  return mass / 3 - 2;
}

fn main() -> io::Result<()> {
    let mut fuel = 0;
    for line in io::stdin().lock().lines() {
        let mass = line?.parse::<i32>().unwrap();
        let module_fuel = fuel_for_mass(mass);
        fuel += module_fuel;
        println!(" {}", module_fuel);
    }
    println!(" {}", fuel);
    Ok(())
}
