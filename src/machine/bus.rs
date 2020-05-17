

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BusAddress(pub u16);

#[derive(Clone, Copy)]
enum VRamBank {
    Bank0,
    // Bank1,
}

#[derive(Clone, Copy)]
enum SystemRamBank {
    Bank1,
    // Bank2,
    // Bank3,
    // Bank4,
    // Bank5,
    // Bank6,
    // Bank7,
}

#[derive(Clone)]
pub struct Bus {
    v_ram_bank: VRamBank,
    system_ram_bank: SystemRamBank,
}


#[allow(dead_code)]
pub mod memory_map {
    use super::BusAddress;
    pub const ROM_BANK_0: (BusAddress, BusAddress) =
        (BusAddress(0x_0000_u16), BusAddress(0x_4000_u16));
    pub const SWITCHABLE_ROM_BANK: (BusAddress, BusAddress) =
        (ROM_BANK_0.1, BusAddress(0x_8000_u16));
    pub const VIDEO_RAM: (BusAddress, BusAddress) =
        (SWITCHABLE_ROM_BANK.1, BusAddress(0x_A000_u16));
    pub const EXTERNAL_RAM: (BusAddress, BusAddress) = (VIDEO_RAM.1, BusAddress(0x_C000_u16));
    pub const SYSTEM_RAM: (BusAddress, BusAddress) = (EXTERNAL_RAM.1, BusAddress(0x_E000_u16));
    pub const PROHIBITED_0: (BusAddress, BusAddress) = (SYSTEM_RAM.1, BusAddress(0x_FE00_u16));
    pub const SPRITE_ATTRIB_MEMORY: (BusAddress, BusAddress) =
        (PROHIBITED_0.1, BusAddress(0x_FEA0_u16));
    pub const PROHIBITED_1: (BusAddress, BusAddress) =
        (SPRITE_ATTRIB_MEMORY.1, BusAddress(0x_FF00_u16));
    pub const IO_PORTS: (BusAddress, BusAddress) = (PROHIBITED_1.1, BusAddress(0x_FF80_u16));
    pub const HIGH_RAM: (BusAddress, BusAddress) = (IO_PORTS.1, BusAddress(0x_FFFF_u16));
    pub const INTERRUPT_ENABLE_REGISTER: BusAddress = HIGH_RAM.1;
}

#[allow(dead_code)]
mod reserved_locations {
    use super::BusAddress;
    pub const RESTART_00_ADDRESS: BusAddress = BusAddress(0x_0000_u16);
    pub const RESTART_08_ADDRESS: BusAddress = BusAddress(0x_0008_u16);
    pub const RESTART_10_ADDRESS: BusAddress = BusAddress(0x_0010_u16);
    pub const RESTART_18_ADDRESS: BusAddress = BusAddress(0x_0018_u16);
    pub const RESTART_20_ADDRESS: BusAddress = BusAddress(0x_0020_u16);
    pub const RESTART_28_ADDRESS: BusAddress = BusAddress(0x_0028_u16);
    pub const RESTART_30_ADDRESS: BusAddress = BusAddress(0x_0030_u16);
    pub const RESTART_38_ADDRESS: BusAddress = BusAddress(0x_0038_u16);
    pub const V_BLANK_INTERRUPT: BusAddress = BusAddress(0x_0040_u16);
    pub const LCDC_STATUS_INTERRUPT: BusAddress = BusAddress(0x_0048_u16);
    pub const TIMER_FLOW_INTERRUPT: BusAddress = BusAddress(0x_0050_u16);
    pub const SERIAL_TRANSFER_COMPLETION_INTERRUPT: BusAddress = BusAddress(0x_0058_u16);
    pub const HIGH_TO_LOW_OF_P10_P13_INTERRUPT: BusAddress = BusAddress(0x_0060_u16);

    pub const EXECUTION_START_POINT: (BusAddress, BusAddress) =
        (BusAddress(0x_0100_u16), BusAddress(0x_0104_u16));
    pub const NINTENDO_GRAPHIC: (BusAddress, BusAddress) =
        (BusAddress(0x_0104_u16), BusAddress(0x_0134_u16));
    pub const GAME_TITLE: (BusAddress, BusAddress) =
        (BusAddress(0x_0134_u16), BusAddress(0x_0143_u16));
    pub const COLOR_GAMEBOY: BusAddress = BusAddress(0x_0143_u16);
    pub const LICENSE_CODE_HIGH_NIBBLE: BusAddress = BusAddress(0x_0144_u16);
    pub const LICENSE_CODE_LOW_NIBBLE: BusAddress = BusAddress(0x_0145_u16);
    pub const SUPER_GAMEBOY: BusAddress = BusAddress(0x_0146_u16);
    pub const CARTRIDGE_TYPE: BusAddress = BusAddress(0x_0147_u16);
    pub const ROM_SIZE: BusAddress = BusAddress(0x_0148_u16);
    pub const RAM_SIZE: BusAddress = BusAddress(0x_0149_u16);
    pub const DESTINATION_CODE: BusAddress = BusAddress(0x_014A_u16);
    pub const LICENSEE_CODE: BusAddress = BusAddress(0x_014B_u16);
    pub const MASK_ROM_VERSION_NUMBER: BusAddress = BusAddress(0x_014C_u16);
    pub const COMPLEMENT_CHECK: BusAddress = BusAddress(0x_014D_u16);
    pub const CHECKSUM: (BusAddress, BusAddress) =
        (BusAddress(0x_014E_u16), BusAddress(0x_0150_u16));
}

mod io_registers {}

impl Bus {
    pub fn new() -> Bus {
        Bus{
            v_ram_bank: VRamBank::Bank0,
            system_ram_bank: SystemRamBank::Bank1,
        }
    }

    pub fn write8(self, machine: &mut super::MachineState, address: BusAddress, byte: u8) {
        use memory_map::*;
        // match address {
        //     BusAddress(x) if let (min, max) = ROM_BANK_0 if min..=max => println!("cart0"),
        //     SWITCHABLE_ROM_BANK.0..=SWITCHABLE_ROM_BANK.1 => println!("cart1"),
        //     VIDEO_RAM.0..=VIDEO_RAM.1 => println!("vram"),
        //     EXTERNAL_RAM.0..=EXTERNAL_RAM.1 => println!("externalRam"),
        //     SYSTEM_RAM.0..=SYSTEM_RAM.1 => println!("systemRam"),
        //     PROHIBITED_0.0..=PROHIBITED_0.1 => println!("prohibited0"),
        //     SPRITE_ATTRIB_MEMORY.0..=SPRITE_ATTRIB_MEMORY.1 => println!("oam"),
        //     PROHIBITED_1.0..=PROHIBITED_1.1 => println!("systemRam"),
        //     IO_PORTS.0..=IO_PORTS.1 => println!("ioPorts"),
        //     HIGH_RAM.0..=HIGH_RAM.1 => println!("highRam"),
        //     INTERRUPT_ENABLE_REGISTER => println!("interruptEnableRegister"),
        // }

        match address {
            BusAddress(x) if inside(x, ROM_BANK_0) => machine.cart.write(address, byte),
            BusAddress(x) if inside(x, SWITCHABLE_ROM_BANK) => machine.cart.write(address, byte),
            BusAddress(x) if inside(x, VIDEO_RAM) => machine.video_ram.write(address, byte),
            BusAddress(x) if inside(x, EXTERNAL_RAM) => machine.cart.write(address, byte),
            BusAddress(x) if inside(x, SYSTEM_RAM) => machine.system_ram.write(address, byte),
            BusAddress(x) if inside(x, PROHIBITED_0) => panic!("write to prohibited 0"),
            BusAddress(x) if inside(x, SPRITE_ATTRIB_MEMORY) => println!("oam"),
            BusAddress(x) if inside(x, PROHIBITED_1) => panic!("write to prohibited 1"),
            BusAddress(x) if inside(x, IO_PORTS) => println!("ioPorts"),
            BusAddress(x) if inside(x, HIGH_RAM) => machine.high_ram.write(address, byte),
            INTERRUPT_ENABLE_REGISTER => println!("interruptEnableRegister"),
            _ => panic!("should not get here"),
        }
    }
}

fn inside(address: u16, (min, max): (BusAddress, BusAddress)) -> bool {
    address >= min.0 && address < max.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let cart = super::load_cart(&[b'a', b'b', b'c']);

        // assert_eq!(cart.rom, [b'a', b'b', b'c']);
    }
}
