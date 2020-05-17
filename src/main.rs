mod machine;

use std::env;
use std::fs;

use machine::Cart;
use machine::MachineState;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    let rom_contents = fs::read(rom_path).expect("Something went wrong reading ROM file");

    let machine = MachineState::new(Cart::new(&rom_contents[..]));

    machine.step_forwards();

    println!("Hello, world!");
}
