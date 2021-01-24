// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
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
