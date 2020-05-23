pub use cart::Cart;

mod bus;
mod cart;
mod cpu;
mod ram;

use bus::memory_map;

/// The state of the machine at one moment
#[derive(Clone)]
pub struct MachineState {
    cart: Cart,
    cpu: cpu::CpuState,
    system_ram: ram::Ram,
    video_ram: ram::Ram,
    high_ram: ram::Ram,
    bus: bus::Bus,
}

/// A reversible mutation to the machine state
pub struct MachineStep {

}

struct MachineMutation {

}

impl MachineState {
    pub fn new(cart: Cart) -> Self {
        MachineState {
            cart: cart,
            cpu: cpu::CpuState::new(),
            system_ram: ram::Ram::new(memory_map::SYSTEM_RAM),
            video_ram: ram::Ram::new(memory_map::VIDEO_RAM),
            high_ram: ram::Ram::new(memory_map::HIGH_RAM),
            bus: bus::Bus::new(),
        }
    }

    /// 
    pub fn step_forwards(self) -> MachineStep {
        panic!("not implemented");
    }

    // pub fn step_fackwards(machine: MachineState, step: MachineStep) -> MachineState {
    //     panic!("not implemented");
    // }

    // fn calculate_mutation(machine: &MachineState) -> MachineMutation {
    //     MachineMutation {

    //     }
    // }

    // fn step_machine(machine: MachineState, mutation: MachineMutation) -> (MachineState, MachineStep) {
    //     panic!("not implemented");
    // }
}
