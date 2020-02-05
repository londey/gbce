mod machine;

use std::env;
use std::fs;

use machine::Cart;
use machine::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    let rom_contents = fs::read(rom_path).expect("Something went wrong reading ROM file");

    let machine = Machine::new(Cart::new(&rom_contents[..]));

    println!("Hello, world!");
}
