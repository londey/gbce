pub struct Cart {
    rom_data: Vec<u8>,
}

impl Cart {
    pub fn new(_rom_data: &[u8]) -> Cart {
        Cart {
            rom_data: Vec::from(_rom_data),
        }
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
