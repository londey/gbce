mod bus;
mod cart;
mod cpu;
mod screen;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    let rom_contents = fs::read(rom_path).expect("Something went wrong reading ROM file");

    let cart = crate::cart::load_cart(&rom_contents[..]);

    let machine = crate::bus::create_machine(cart);

    println!("Hello, world!");
}
