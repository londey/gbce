pub struct BusAddress(u16);

struct Ram {
    bytes: Vec<u8>,
}


struct Bus {
    _cpu: crate::cpu::Cpu,

}

mod memory_map {
    use super::BusAddress;
    const ROM_BANK_0: (BusAddress, BusAddress) = (BusAddress(0x_0000_u16), BusAddress(0x_4000_u16));
    const SWITCHABLE_ROM_BANK: (BusAddress, BusAddress) = (ROM_BANK_0.1, BusAddress(0x_8000_u16));
    const VIDEO_RAM: (BusAddress, BusAddress) = (SWITCHABLE_ROM_BANK.1, BusAddress(0x_A000_u16));
    const SWITCHABLE_RAM: (BusAddress, BusAddress) = (VIDEO_RAM.1, BusAddress(0x_C000_u16));
    const INTERNAL_RAM_1: (BusAddress, BusAddress) = (SWITCHABLE_RAM.1, BusAddress(0x_E000_u16));
    const INTERNAL_RAM_ECHO: (BusAddress, BusAddress) = (INTERNAL_RAM_1.1, BusAddress(0x_FE00_u16));
    const SPRITE_ATTRIB_MEMORY: (BusAddress, BusAddress) = (INTERNAL_RAM_ECHO.1, BusAddress(0x_FEA0_u16));
    const EMPTY_IO_1: (BusAddress, BusAddress) = (SPRITE_ATTRIB_MEMORY.1, BusAddress(0x_FF00_u16));
    const IO_PORTS: (BusAddress, BusAddress) = (EMPTY_IO_1.1, BusAddress(0x_FF4C_u16));
    const EMPTY_IO_2: (BusAddress, BusAddress) = (IO_PORTS.1, BusAddress(0x_FF80_u16));
    const INTERNAL_RAM_2: (BusAddress, BusAddress) = (EMPTY_IO_2.1, BusAddress(0x_FFFF_u16));
    const INTERRUPT_ENABLE_REGISTER: BusAddress = INTERNAL_RAM_2.1;
}

mod reserved_locations {
    use super::BusAddress;
    const RESTART_00_ADDRESS: BusAddress = BusAddress(0x_0000_u16);
    const RESTART_08_ADDRESS: BusAddress = BusAddress(0x_0008_u16);
    const RESTART_10_ADDRESS: BusAddress = BusAddress(0x_0010_u16);
    const RESTART_18_ADDRESS: BusAddress = BusAddress(0x_0018_u16);
    const RESTART_20_ADDRESS: BusAddress = BusAddress(0x_0020_u16);
    const RESTART_28_ADDRESS: BusAddress = BusAddress(0x_0028_u16);
    const RESTART_30_ADDRESS: BusAddress = BusAddress(0x_0030_u16);
    const RESTART_38_ADDRESS: BusAddress = BusAddress(0x_0038_u16);
    const V_BLANK_INTERRUPT: BusAddress = BusAddress(0x_0040_u16);
    const LCDC_STATUS_INTERRUPT: BusAddress = BusAddress(0x_0048_u16);
    const TIMER_FLOW_INTERRUPT: BusAddress = BusAddress(0x_0050_u16);
    const SERIAL_TRANSFER_COMPLETION_INTERRUPT: BusAddress = BusAddress(0x_0058_u16);
    const HIGH_TO_LOW_OF_P10_P13_INTERRUPT: BusAddress = BusAddress(0x_0060_u16);

    const EXECUTION_START_POINT: (BusAddress, BusAddress) = (BusAddress(0x_0100_u16), BusAddress(0x_0104_u16));
    const NINTENDO_GRAPHIC: (BusAddress, BusAddress) = (BusAddress(0x_0104_u16), BusAddress(0x_0134_u16));
    const GAME_TITLE: (BusAddress, BusAddress) = (BusAddress(0x_0134_u16), BusAddress(0x_0143_u16));
    const COLOR_GAMEBOY: BusAddress = BusAddress(0x_0143_u16);
    const LICENSE_CODE_HIGH_NIBBLE: BusAddress = BusAddress(0x_0144_u16);
    const LICENSE_CODE_LOW_NIBBLE: BusAddress = BusAddress(0x_0145_u16);
    const SUPER_GAMEBOY: BusAddress = BusAddress(0x_0146_u16);
    const CARTRIDGE_TYPE: BusAddress = BusAddress(0x_0147_u16);
    const ROM_SIZE: BusAddress = BusAddress(0x_0148_u16);
    const RAM_SIZE: BusAddress = BusAddress(0x_0149_u16);
    const DESTINATION_CODE: BusAddress = BusAddress(0x_014A_u16);
    const LICENSEE_CODE: BusAddress = BusAddress(0x_014B_u16);
    const MASK_ROM_VERSION_NUMBER: BusAddress = BusAddress(0x_014C_u16);
    const COMPLEMENT_CHECK: BusAddress = BusAddress(0x_014D_u16);
    const CHECKSUM: (BusAddress, BusAddress) = (BusAddress(0x_014E_u16), BusAddress(0x_0150_u16));


}

mod io_registers {
    
}


impl Bus {
    fn write8(address: BusAddress, byte: u8) {

    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let cart = super::load_cart(&[b'a', b'b', b'c']);

        // assert_eq!(cart.rom, [b'a', b'b', b'c']);
    }
}
