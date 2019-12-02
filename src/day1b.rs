use std::io::{self, BufRead};

fn fuel_for_mass(mass: i32) -> i32 {
  let fuel = mass / 3 - 2;
  if fuel <= 0 {
      return 0;
  }
  return fuel + fuel_for_mass(fuel);
}

fn fuel_for_modules() -> io::Result<i32> {
    let mut fuel = 0;
    for line in io::stdin().lock().lines() {
        let mass = line?.parse::<i32>().unwrap();
        let module_fuel = fuel_for_mass(mass);
        fuel += module_fuel;
        println!(" {} {}", mass, module_fuel);
    }
    return Ok(fuel);
}

fn main() -> io::Result<()> {
    let fuel = fuel_for_modules()?;
    println!("{}", fuel);
    Ok(())
}
