use std::convert::TryInto;

#[derive(Copy, Clone)]
pub struct BusAddress(u16);

pub struct Ram {
    mapping: (BusAddress, BusAddress),
    bytes: Vec<u8>,
}

fn create_ram((begin, end): (BusAddress, BusAddress)) -> Ram {
    Ram {
        mapping: (begin, end),
        bytes: vec![0; (end.0 - begin.0).try_into().unwrap()],
    }
}

pub fn create_machine(cart: crate::cart::Cart) -> Bus {
    Bus {
        cart: cart,
        cpu: crate::cpu::create_cpu(),
        main_ram: create_ram(memory_map::MAIN_RAM),
        video_ram: create_ram(memory_map::VIDEO_RAM),
    }
}

pub struct Bus {
    cart: crate::cart::Cart,
    cpu: crate::cpu::Cpu,
    main_ram: Ram,
    video_ram: Ram,
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
    pub const SWITCHABLE_RAM: (BusAddress, BusAddress) = (VIDEO_RAM.1, BusAddress(0x_C000_u16));
    pub const MAIN_RAM: (BusAddress, BusAddress) = (SWITCHABLE_RAM.1, BusAddress(0x_E000_u16));
    pub const INTERNAL_RAM_ECHO: (BusAddress, BusAddress) = (MAIN_RAM.1, BusAddress(0x_FE00_u16));
    pub const SPRITE_ATTRIB_MEMORY: (BusAddress, BusAddress) =
        (INTERNAL_RAM_ECHO.1, BusAddress(0x_FEA0_u16));
    pub const EMPTY_IO_1: (BusAddress, BusAddress) =
        (SPRITE_ATTRIB_MEMORY.1, BusAddress(0x_FF00_u16));
    pub const IO_PORTS: (BusAddress, BusAddress) = (EMPTY_IO_1.1, BusAddress(0x_FF4C_u16));
    pub const EMPTY_IO_2: (BusAddress, BusAddress) = (IO_PORTS.1, BusAddress(0x_FF80_u16));
    pub const INTERNAL_RAM_2: (BusAddress, BusAddress) = (EMPTY_IO_2.1, BusAddress(0x_FFFF_u16));
    pub const INTERRUPT_ENABLE_REGISTER: BusAddress = INTERNAL_RAM_2.1;
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
    fn write8(address: BusAddress, byte: u8) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let cart = super::load_cart(&[b'a', b'b', b'c']);

        // assert_eq!(cart.rom, [b'a', b'b', b'c']);
    }
}
