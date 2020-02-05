mod bus;
mod cart;
mod cpu;
mod ram;

use bus::memory_map;
use bus::Bus;
pub use cart::Cart;
use cpu::Cpu;
use ram::Ram;

pub struct Machine {
    cart: Cart,
    cpu: Cpu,
    system_ram: Ram,
    video_ram: Ram,
    high_ram: Ram,
    bus: Bus,
}

impl Machine {
    pub fn new(cart: Cart) -> Self {
        Machine {
            cart: cart,
            cpu: Cpu::new(),
            system_ram: Ram::new(memory_map::SYSTEM_RAM),
            video_ram: Ram::new(memory_map::VIDEO_RAM),
            high_ram: Ram::new(memory_map::HIGH_RAM),
            bus: Bus::new(),
        }
    }
}
