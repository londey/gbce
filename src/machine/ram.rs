use super::bus::BusAddress;
use std::convert::TryInto;

pub struct Ram {
    mapping: (BusAddress, BusAddress),
    bytes: Vec<u8>,
}

impl Ram {
    pub fn new((begin, end): (BusAddress, BusAddress)) -> Ram {
        Ram {
            mapping: (begin, end),
            bytes: vec![0; (end.0 - begin.0).try_into().unwrap()],
        }
    }

    pub fn write(&mut self, address: BusAddress, byte: u8) {
        self.bytes[address.0 as usize] = byte
    }

    pub fn read(&mut self, address: BusAddress) -> u8 {
        self.bytes[address.0 as usize]
    }  
}
