use std::convert::TryInto;

pub trait ArrayAsInt {
    fn as_usize(&self) -> usize;
    fn as_u32(&self) -> u32;
}

impl ArrayAsInt for [u8] {
    fn as_usize(&self) -> usize {
        let temp: [u8; 4] = self.try_into().unwrap();
        u32::from_be_bytes(temp) as usize
    }

    fn as_u32(&self) -> u32 {
        let temp: [u8; 4] = self.try_into().unwrap();
        u32::from_be_bytes(temp)
    }
}
