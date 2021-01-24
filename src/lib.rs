// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str;
use std::string::String;

mod errors;
use errors::{DSStoreError, Result};

mod impls;
use impls::ArrayAsInt;

mod record;
use record::Record;

const MAGIC: [u8; 8] = [0x00, 0x00, 0x00, 0x01, 0x42, 0x75, 0x64, 0x31];

pub struct DSStore {
    data: Vec<u8>,

    position: usize,

    pub offsets: Vec<usize>,
    pub table_of_contents: HashMap<String, usize>,
    pub free_list: BTreeMap<usize, Vec<usize>>,
    pub internal_block_levels: usize,
    pub record_count: usize,
    pub block_count: usize,
    pub records: Vec<Record>,
}

impl DSStore {
    pub fn new(data: Vec<u8>) -> Self {
        DSStore {
            data: data,
            position: 0,
            offsets: Vec::new(),
            table_of_contents: HashMap::new(),
            free_list: BTreeMap::new(),
            internal_block_levels: 0,
            record_count: 0,
            block_count: 0,
            records: Vec::new(),
        }
    }

    pub fn parse(mut self) -> crate::Result<Self> {
        if self.read(8)? != &MAGIC {
            return Err(crate::DSStoreError::BadMagic);
        }

        let root_offset = self.parse_header()?;
        self.position = root_offset;

        self.parse_root()?;
        self.parse_tree()?;

        Ok(self)
    }

    fn parse_header(&mut self) -> crate::Result<usize> {
        let root_offset = self.read(4)?;
        self.skip(4)?;

        if root_offset != self.read(4)? {
            return Err(crate::DSStoreError::OffsetsDontMatch);
        }

        self.skip(16)?;

        Ok(root_offset.as_usize() + 0x04)
    }

    fn parse_root(&mut self) -> crate::Result<()> {
        //
        // Offsets
        //
        let start_of_block = self.position;

        let offset_count = self.read(4)?.as_usize();

        self.skip(4)?;

        for _ in 0..offset_count {
            let value = self.read(4)?.as_usize();
            self.offsets.push(value);
        }

        // "[T]he padding is aligned to go up to the next multiple of 256 entries (1024 bytes).
        self.jump_to((start_of_block - 0x04) + (offset_count * 8) + (256 - offset_count) * 4)?;

        //
        // Table of content
        //
        let toc_count = self.read(4)?.as_usize();

        for _ in 0..toc_count {
            let name_len = self.read_one()? as usize;

            let key = str::from_utf8(&self.read(name_len)?)?.to_string();
            let address = self.read(4)?.as_usize();

            self.table_of_contents.insert(key, address);
        }

        //
        // Free list
        //
        for i in 0..32 {
            let key = (2 as usize).pow(i);
            let item_count = self.read(4)?.as_usize();

            let mut items: Vec<usize> = Vec::new();

            for _ in 0..item_count {
                items.push(self.read(4)?.as_usize())
            }

            self.free_list.insert(key, items);
        }

        Ok(())
    }

    fn parse_tree(&mut self) -> crate::Result<()> {
        let first_block = self.parse_tree_root()?;

        self.jump_to(first_block)?;

        println!("{:#x}", self.position);

        Ok(())
    }

    fn parse_tree_root(&mut self) -> crate::Result<usize> {
        let first_block_id = self.table_of_contents["DSDB"];
        let first_block_address = self.offsets[first_block_id] >> 0x5 << 0x5;

        println!("{:#x}, {:#x}", first_block_id, first_block_address);

        self.jump_to(first_block_address)?;

        let first_data_block = self.read(4)?.as_usize();
        let first_data_block_address = first_data_block >> 0x5 << 0x5;

        self.internal_block_levels = self.read(4)?.as_usize();
        self.record_count = self.read(4)?.as_usize();
        self.block_count = self.read(4)?.as_usize();

        Ok(first_data_block_address)
    }

    fn parse_record(&mut self) -> crate::Result<()> {
        let length = self.read(4)?.as_usize();
        let file_name = self.read_utf16_str(length * 2)?;
        let structure_id = self.read(4)?.as_usize();

        let str_type = self.read(4)?;
        let structure_type = str::from_utf8(&str_type)?;

        self.records.push(Record::new(
            file_name,
            structure_type.to_string(),
            structure_id,
        ));

        Ok(())
    }

    fn read(&mut self, count: usize) -> crate::Result<Vec<u8>> {
        let end_index = self.position + count;
        if self.data.len() < end_index {
            return Err(crate::DSStoreError::TooLittleData(end_index));
        }

        let temp = &self.data[self.position..end_index].to_vec();
        self.position = end_index;

        Ok(temp.clone())
    }

    fn read_utf16_str(&mut self, count: usize) -> crate::Result<String> {
        let end_index = self.position + count;
        if self.data.len() < end_index {
            return Err(crate::DSStoreError::TooLittleData(end_index));
        }

        let u8s = &self.data[self.position..end_index].to_vec();
        let mut inter: [u8; 2] = [0, 0];
        let mut temp: Vec<u16> = Vec::new();

        for ch in u8s.chunks(2) {
            inter.copy_from_slice(ch);
            temp.push(u16::from_be_bytes(inter))
        }

        Ok(String::from_utf16(&temp)?)
    }

    fn read_one(&mut self) -> crate::Result<u8> {
        if self.data.len() < self.position + 1 {
            return Err(crate::DSStoreError::TooLittleData(self.position + 1));
        }

        self.position += 1;

        Ok(self.data[self.position - 1])
    }

    fn skip(&mut self, count: usize) -> crate::Result<()> {
        let end_index = self.position + count;
        if self.data.len() < end_index {
            return Err(crate::DSStoreError::TooLittleData(end_index));
        }

        self.position = end_index;

        Ok(())
    }

    fn jump_to(&mut self, to: usize) -> crate::Result<()> {
        if self.data.len() < to {
            return Err(crate::DSStoreError::TooLittleData(to));
        }

        self.position = to;

        Ok(())
    }
}
