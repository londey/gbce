
use super::bus::BusAddress;


pub struct Cart {
    rom_data: Vec<u8>,
}



impl Cart {
    pub fn new(_rom_data: &[u8]) -> Cart {
        Cart {
            rom_data: Vec::from(_rom_data),
        }
    }

    pub fn write(&mut self, address: BusAddress, byte: u8) {

    }

    pub fn read(&mut self, address: BusAddress) -> u8 {
        self.rom_data[address.0 as usize]
    }    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let cart = super::Cart::new(&[b'a', b'b', b'c']);

        assert_eq!(cart.rom_data, [b'a', b'b', b'c']);
    }
}
