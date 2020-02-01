pub fn load_cart(_rom_data: &[u8]) -> Cart {
    Cart {
        rom: Vec::from(_rom_data),
    }
}

pub struct Cart {
    rom: Vec<u8>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let cart = super::load_cart(&[b'a', b'b', b'c']);

        assert_eq!(cart.rom, [b'a', b'b', b'c']);
    }
}
